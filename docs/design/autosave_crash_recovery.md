# Autosave & Crash Recovery

This document describes the autosave system used to periodically snapshot the simulation and recover after an unclean shutdown.

## Goals

- **Periodic autosaves** on a fixed cadence (configurable interval)
- **Rotating slots** so recent history is preserved (configurable slot count)
- **Crash detection + recovery prompt** in the CLI
- **Corruption tolerance**: recover the newest valid save even if the latest slot is truncated/corrupt

## Determinism

Autosave cadence is based on **simulation ticks** (`systems::Time::ticks`), not wall-clock time.

This keeps the simulation deterministic: the decision to autosave depends only on the tick counter.

## File Layout

Autosaves are stored in a directory (default: `autosaves/`).

- **Primary slot**: `autosave-{slot}.{ext}`
- **Per-slot backup**: `autosave-{slot}.{ext}.bak`
- **Crash marker**: `autosave.lock`

Where:

- `{slot}` is the rotating slot index (0..N-1)
- `{ext}` is one of `json`, `ron`, `cbor`

## Atomic Writes and Backups

Writing a slot uses a simple atomic pattern:

1. Write `autosave-{slot}.{ext}.tmp`
2. Rename existing `autosave-{slot}.{ext}` → `autosave-{slot}.{ext}.bak` (best-effort)
3. Rename `.tmp` → the final slot path

If a crash happens mid-write, recovery can fall back to `.bak`.

## Recovery

Recovery scans all configured slots and their `.bak` backups, decodes any valid saves, and picks the **newest** by `SaveGame.ticks`.

This avoids relying on filesystem modification times (which can be inconsistent across platforms).

## CLI Usage

### Enable autosave during a demo run

```bash
cargo run -p gc_cli -- jobs --autosave-every 10 --autosave-slots 3 --autosave-dir autosaves
```

### Load the newest autosave before running a demo

```bash
cargo run -p gc_cli -- jobs --load-autosave --autosave-dir autosaves
```

### Crash recovery prompt

If `autosave.lock` exists and the CLI is running interactively, `gc_cli` will prompt to recover the newest valid autosave.

## Tests

- **Golden fixtures** live under `crates/gc_core/tests/fixtures/autosave/`
- **Recovery tests** validate:
  - rotating slots write correctly
  - recovery skips corrupted primaries and can recover from `.bak`


