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
    MASTER_VOLUME: 0.3,
    MUSIC_VOLUME: 0.2,
    SFX_VOLUME: 0.4,
    BASE_BPM: 120,
    MAX_BPM: 180,
    
    // Sound effect frequencies
    EAT_FREQ_1: 800,
    EAT_FREQ_2: 1000,
    GAME_OVER_FREQ_1: 400,
    GAME_OVER_FREQ_2: 300,
    GAME_OVER_FREQ_3: 200,
    
    // Sound durations
    EAT_DURATION: 0.1,
    GAME_OVER_DURATION: 0.2
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