/**
 * SNEK Game Configuration
 * Centralized configuration object for all game settings
 */

export const CONFIG = {
  VISUAL: {
    GRID_SIZE: 20,
    CANVAS_WIDTH: 800,
    CANVAS_HEIGHT: 600,
    GLOW_INTENSITY: 1.0,
    PARTICLE_COUNT: 15,
    COLORS: {
      SNAKE: '#00ff41',
      FOOD: '#66ff66',
      BACKGROUND: '#000608',
      GRID: '#001a06',
      GLOW: '#00ff4180'
    }
  },
  
  GAMEPLAY: {
    BASE_SPEED: 200,
    SPEED_INCREASE_FACTOR: 0.95,
    MIN_SPEED: 80,
    POINTS_PER_FOOD: 10,
    LEVEL_THRESHOLD: 5
  },
  
  AUDIO: {
    MASTER_VOLUME: 0.4,
    MUSIC_VOLUME: 0.3,
    SFX_VOLUME: 0.5,
    BASE_BPM: 120,
    MAX_BPM: 180,
    
    // Dark synthwave settings
    MENU_BPM: 70,        // Slow, atmospheric menu music
    GAME_MIN_BPM: 120,   // Starting gameplay tempo
    GAME_MAX_BPM: 180,   // Maximum intensity tempo
    
    // Reverb and effects
    REVERB_WET: 0.25,
    DELAY_WET: 0.2,
    DELAY_FEEDBACK: 0.3,
    
    // Channel mix levels for dark synthwave
    BASS_VOLUME: 0.9,
    LEAD_VOLUME: 0.7,
    PAD_VOLUME: 0.5,
    DRUMS_VOLUME: 0.8,
    ARP_VOLUME: 0.4,
    SFX_VOLUME: 0.6,
    
    // Musical keys and scales
    ROOT_KEY: 'Am',      // A minor for dark atmosphere
    HARMONIC_MINOR: true, // Use harmonic minor for evil sound
    PHRYGIAN_MODE: true,  // Add exotic darkness
    
    // Sound effect frequencies (cyberpunk style)
    EAT_FREQ_1: 800,
    EAT_FREQ_2: 1200,
    GAME_OVER_FREQ_1: 220, // A2 - dark and ominous
    GAME_OVER_FREQ_2: 174, // F2
    GAME_OVER_FREQ_3: 146, // D2
    
    // Sound durations
    EAT_DURATION: 0.15,
    GAME_OVER_DURATION: 0.8,
    
    // Intensity scaling
    INTENSITY_SCORE_MAX: 1000,  // Score needed for max intensity
    INTENSITY_LENGTH_MAX: 50    // Snake length for max intensity
  },
  
  EXCLAMATIONS: {
    MESSAGES: [
      'ALL RIIIIGHT!',
      'NICE!',
      'SNEK!!!!',
      'WOW!!!',
      'EPIC!',
      'RADICAL!',
      'GNARLY!',
      'SICK!'
    ],
    TRIGGERS: {
      SCORE_MULTIPLES: [50, 100, 200, 300, 500],
      SNAKE_LENGTHS: [10, 20, 30, 50]
    },
    DISPLAY_DURATION: 2000,
    ANIMATION_DURATION: 1000
  },
  
  PARTICLES: {
    LIFETIME: 1000,
    SPEED_MIN: 50,
    SPEED_MAX: 150,
    SIZE_MIN: 2,
    SIZE_MAX: 6
  },
  
  HIGH_SCORES: {
    MAX_SCORES: 10,
    STORAGE_KEY: 'snekHighScores',
    DEFAULT_SCORES: [
      { name: 'CPU', score: 500 },
      { name: 'PRO', score: 400 },
      { name: 'ADV', score: 300 },
      { name: 'INT', score: 200 },
      { name: 'BEG', score: 100 },
      { name: 'SNK', score: 50 }
    ]
  },
  
  CONTROLS: {
    UP: ['ArrowUp', 'KeyW'],
    DOWN: ['ArrowDown', 'KeyS'],
    LEFT: ['ArrowLeft', 'KeyA'],
    RIGHT: ['ArrowRight', 'KeyD'],
    PAUSE: ['Space', 'KeyP'],
    MUTE: ['KeyM'],
    RESTART: ['KeyR']
  }
};

export default CONFIG;