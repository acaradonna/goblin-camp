//! Autosave and crash recovery helpers.
//!
//! This module builds on `crate::save` by providing:
//! - Periodic autosaves on a deterministic tick cadence (based on `systems::Time::ticks`)
//! - Rotating save slots on disk
//! - Recovery helpers that pick the newest valid save (by in-save tick counter)
//!
//! Notes:
//! - Autosave cadence is based on simulation ticks, not wall-clock time, to preserve determinism.
//! - File IO is performed with a simple atomic-write pattern that leaves a per-slot `.bak` backup.

use crate::save::{self, SaveGame};
use crate::systems::Time;
use bevy_ecs::prelude::World;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Supported codecs for autosave files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveCodec {
    Json,
    Ron,
    Cbor,
}

impl SaveCodec {
    pub fn extension(self) -> &'static str {
        match self {
            SaveCodec::Json => "json",
            SaveCodec::Ron => "ron",
            SaveCodec::Cbor => "cbor",
        }
    }
}

/// Autosave configuration.
#[derive(Debug, Clone)]
pub struct AutosaveConfig {
    /// Save every N simulation ticks (0 disables autosave).
    pub every_ticks: u64,
    /// Number of rotating slots to keep (0 disables autosave).
    pub slots: usize,
    /// Codec used for save bytes on disk.
    pub codec: SaveCodec,
    /// File prefix used for slot naming (e.g. `autosave-0.json`).
    pub base_name: String,
}

impl AutosaveConfig {
    pub fn enabled(&self) -> bool {
        self.every_ticks > 0 && self.slots > 0
    }
}

impl Default for AutosaveConfig {
    fn default() -> Self {
        Self {
            every_ticks: 0,
            slots: 3,
            codec: SaveCodec::Json,
            base_name: "autosave".to_string(),
        }
    }
}

/// Result returned by recovery helpers.
#[derive(Debug, Clone)]
pub struct RecoveredAutosave {
    pub save: SaveGame,
    pub path: PathBuf,
}

/// Errors for autosave IO and codec operations.
#[derive(Debug, Error)]
pub enum AutosaveError {
    #[error("autosave requires a Time resource in the World")]
    MissingTime,
    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("json codec error at {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("ron codec error at {path}: {source}")]
    Ron {
        path: PathBuf,
        #[source]
        source: ron::Error,
    },
    #[error("cbor encode error at {path}: {source}")]
    CborEncode {
        path: PathBuf,
        #[source]
        source: ciborium::ser::Error<io::Error>,
    },
    #[error("cbor decode error at {path}: {source}")]
    CborDecode {
        path: PathBuf,
        #[source]
        source: ciborium::de::Error<io::Error>,
    },
}

/// Manages rotating autosave slots.
#[derive(Debug)]
pub struct AutosaveManager {
    dir: PathBuf,
    config: AutosaveConfig,
    next_slot: usize,
    last_saved_tick: u64,
}

impl AutosaveManager {
    pub fn new(dir: impl Into<PathBuf>, config: AutosaveConfig) -> Result<Self, AutosaveError> {
        let dir = dir.into();
        if config.enabled() {
            fs::create_dir_all(&dir).map_err(|e| AutosaveError::Io {
                path: dir.clone(),
                source: e,
            })?;
        }

        Ok(Self {
            dir,
            config,
            next_slot: 0,
            last_saved_tick: 0,
        })
    }

    pub fn config(&self) -> &AutosaveConfig {
        &self.config
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn slot_path(&self, slot: usize) -> PathBuf {
        autosave_slot_path(&self.dir, &self.config.base_name, slot, self.config.codec)
    }

    pub fn slot_backup_path(&self, slot: usize) -> PathBuf {
        autosave_slot_backup_path(&self.dir, &self.config.base_name, slot, self.config.codec)
    }

    /// Create an autosave if the cadence is due.
    ///
    /// Returns `Ok(Some(path))` when a save was written.
    pub fn maybe_autosave(&mut self, world: &mut World) -> Result<Option<PathBuf>, AutosaveError> {
        if !self.config.enabled() {
            return Ok(None);
        }

        let ticks = world
            .get_resource::<Time>()
            .map(|t| t.ticks)
            .ok_or(AutosaveError::MissingTime)?;

        if ticks < self.last_saved_tick.saturating_add(self.config.every_ticks) {
            return Ok(None);
        }

        let save_game = save::save_world(world);
        let slot = self.next_slot % self.config.slots;
        let path = self.slot_path(slot);
        write_save_atomic(&path, &save_game, self.config.codec)?;

        self.last_saved_tick = ticks;
        self.next_slot = (slot + 1) % self.config.slots;

        Ok(Some(path))
    }

    /// Align internal cadence tracking to the world's current tick.
    ///
    /// This is useful when starting from a recovered save (non-zero tick count),
    /// to avoid immediately writing another autosave on the next tick.
    pub fn sync_last_saved_tick_from_world(&mut self, world: &World) -> Result<(), AutosaveError> {
        let ticks = world
            .get_resource::<Time>()
            .map(|t| t.ticks)
            .ok_or(AutosaveError::MissingTime)?;
        self.last_saved_tick = ticks;
        Ok(())
    }
}

/// Compute the on-disk path for a slot.
pub fn autosave_slot_path(dir: &Path, base_name: &str, slot: usize, codec: SaveCodec) -> PathBuf {
    dir.join(format!("{base_name}-{slot}.{}", codec.extension()))
}

/// Compute the `.bak` path for a slot.
pub fn autosave_slot_backup_path(
    dir: &Path,
    base_name: &str,
    slot: usize,
    codec: SaveCodec,
) -> PathBuf {
    let primary = autosave_slot_path(dir, base_name, slot, codec);
    let file_name = primary
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| format!("{base_name}-{slot}.{}", codec.extension()));
    primary.with_file_name(format!("{file_name}.bak"))
}

/// Recover the newest valid autosave (highest `SaveGame.ticks`) from a slot set.
///
/// This scans both primary slot files and their `.bak` backups, skipping any
/// corrupted/unreadable entries.
pub fn recover_latest_autosave(
    dir: impl AsRef<Path>,
    config: &AutosaveConfig,
) -> Result<Option<RecoveredAutosave>, AutosaveError> {
    if config.slots == 0 {
        return Ok(None);
    }

    let dir = dir.as_ref();
    let mut best: Option<RecoveredAutosave> = None;

    for slot in 0..config.slots {
        let candidates = [
            autosave_slot_path(dir, &config.base_name, slot, config.codec),
            autosave_slot_backup_path(dir, &config.base_name, slot, config.codec),
        ];

        for path in candidates {
            if !path.exists() {
                continue;
            }

            let save = match read_save(&path, config.codec) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let replace = match &best {
                None => true,
                Some(current) => {
                    save.ticks > current.save.ticks
                        || (save.ticks == current.save.ticks && path < current.path)
                }
            };

            if replace {
                best = Some(RecoveredAutosave { save, path });
            }
        }
    }

    Ok(best)
}

fn read_save(path: &Path, codec: SaveCodec) -> Result<SaveGame, AutosaveError> {
    match codec {
        SaveCodec::Json => {
            let data = fs::read_to_string(path).map_err(|e| AutosaveError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
            save::decode_json(&data).map_err(|e| AutosaveError::Json {
                path: path.to_path_buf(),
                source: e,
            })
        }
        SaveCodec::Ron => {
            let data = fs::read_to_string(path).map_err(|e| AutosaveError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
            save::decode_ron(&data).map_err(|e| AutosaveError::Ron {
                path: path.to_path_buf(),
                source: e,
            })
        }
        SaveCodec::Cbor => {
            let bytes = fs::read(path).map_err(|e| AutosaveError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
            save::decode_cbor(&bytes).map_err(|e| AutosaveError::CborDecode {
                path: path.to_path_buf(),
                source: e,
            })
        }
    }
}

fn encode_save(
    save_game: &SaveGame,
    codec: SaveCodec,
    path: &Path,
) -> Result<Vec<u8>, AutosaveError> {
    match codec {
        SaveCodec::Json => save::encode_json(save_game)
            .map(|s| s.into_bytes())
            .map_err(|e| AutosaveError::Json {
                path: path.to_path_buf(),
                source: e,
            }),
        SaveCodec::Ron => save::encode_ron(save_game)
            .map(|s| s.into_bytes())
            .map_err(|e| AutosaveError::Ron {
                path: path.to_path_buf(),
                source: e,
            }),
        SaveCodec::Cbor => save::encode_cbor(save_game).map_err(|e| AutosaveError::CborEncode {
            path: path.to_path_buf(),
            source: e,
        }),
    }
}

fn write_save_atomic(
    path: &Path,
    save_game: &SaveGame,
    codec: SaveCodec,
) -> Result<(), AutosaveError> {
    let bytes = encode_save(save_game, codec, path)?;
    write_bytes_atomic(path, &bytes)
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<(), AutosaveError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|e| AutosaveError::Io {
        path: parent.to_path_buf(),
        source: e,
    })?;

    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "autosave".to_string());
    let tmp_path = path.with_file_name(format!("{file_name}.tmp"));
    let backup_path = path.with_file_name(format!("{file_name}.bak"));

    {
        let mut f = fs::File::create(&tmp_path).map_err(|e| AutosaveError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;
        f.write_all(bytes).map_err(|e| AutosaveError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;
        f.sync_all().map_err(|e| AutosaveError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;
    }

    // Move the previous save out of the way (best-effort), preserving a rollback.
    if path.exists() {
        let _ = fs::remove_file(&backup_path);
        fs::rename(path, &backup_path).map_err(|e| AutosaveError::Io {
            path: backup_path.clone(),
            source: e,
        })?;
    }

    fs::rename(&tmp_path, path).map_err(|e| AutosaveError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    Ok(())
}
