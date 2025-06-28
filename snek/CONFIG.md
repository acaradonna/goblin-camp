# SNEK Configuration Guide 🐍⚙️

This guide explains how to customize SNEK using the centralized CONFIG system. All game settings are organized in the `CONFIG` object for easy modification.

## 🎨 Visual Configuration

### Basic Visual Settings
```javascript
CONFIG.VISUAL = {
    GRID_SIZE: 20,           // Size of each grid cell (pixels)
    CANVAS_WIDTH: 800,       // Game canvas width
    CANVAS_HEIGHT: 600,      // Game canvas height
    GLOW_INTENSITY: 1.0,     // Global glow effect intensity
    PARTICLE_COUNT: 15       // Base number of particles per explosion
};
```

### Color Scheme
```javascript
CONFIG.VISUAL.COLORS = {
    SNAKE: '#00ff41',        // Snake body color (neon green)
    FOOD: '#66ff66',         // Food color (bright green)  
    BACKGROUND: '#000608',   // Canvas background (dark green)
    GRID: '#001a06',         // Grid background (darker green)
    GLOW: '#00ff4180'        // Glow effect color (transparent green)
};
```

#### Creating Custom Color Themes
```javascript
// Synthwave Purple Theme
CONFIG.VISUAL.COLORS = {
    SNAKE: '#ff00ff',
    FOOD: '#ff66ff', 
    BACKGROUND: '#0a0020',
    GRID: '#1a0040',
    GLOW: '#ff00ff80'
};

// Cyberpunk Blue Theme  
CONFIG.VISUAL.COLORS = {
    SNAKE: '#00ffff',
    FOOD: '#66ffff',
    BACKGROUND: '#000820', 
    GRID: '#001040',
    GLOW: '#00ffff80'
};

// Retro Orange Theme
CONFIG.VISUAL.COLORS = {
    SNAKE: '#ff4400',
    FOOD: '#ff8844',
    BACKGROUND: '#200800',
    GRID: '#401000', 
    GLOW: '#ff440080'
};
```

## 🎮 Gameplay Configuration

### Speed and Difficulty
```javascript
CONFIG.GAMEPLAY = {
    BASE_SPEED: 200,                    // Starting game speed (ms between moves)
    SPEED_INCREASE_FACTOR: 0.95,        // Speed multiplier per growth (0.95 = 5% faster)
    MIN_SPEED: 80,                      // Maximum speed (minimum ms between moves)
    POINTS_PER_FOOD: 10,                // Score per food eaten
    LEVEL_THRESHOLD: 5                  // Snake segments needed per level
};
```

#### Difficulty Presets
```javascript
// Easy Mode
CONFIG.GAMEPLAY = {
    BASE_SPEED: 300,
    SPEED_INCREASE_FACTOR: 0.98,  // Slower acceleration
    MIN_SPEED: 150,
    POINTS_PER_FOOD: 10,
    LEVEL_THRESHOLD: 8            // Longer levels
};

// Hard Mode  
CONFIG.GAMEPLAY = {
    BASE_SPEED: 150,
    SPEED_INCREASE_FACTOR: 0.92,  // Faster acceleration
    MIN_SPEED: 50,
    POINTS_PER_FOOD: 20,          // Higher scoring
    LEVEL_THRESHOLD: 3            // Shorter levels
};

// Insane Mode
CONFIG.GAMEPLAY = {
    BASE_SPEED: 100,
    SPEED_INCREASE_FACTOR: 0.90,
    MIN_SPEED: 30,
    POINTS_PER_FOOD: 50,
    LEVEL_THRESHOLD: 2
};
```

## 🎵 Audio Configuration

### Volume Settings
```javascript
CONFIG.AUDIO = {
    MASTER_VOLUME: 0.3,      // Overall volume (0.0 - 1.0)
    MUSIC_VOLUME: 0.2,       // Music track volume
    SFX_VOLUME: 0.4,         // Sound effects volume
    BASE_BPM: 120,           // Starting music tempo
    MAX_BPM: 180             // Maximum music tempo
};
```

### Audio Frequencies
```javascript
CONFIG.AUDIO = {
    EAT_FREQ_1: 800,         // First eating sound frequency (Hz)
    EAT_FREQ_2: 1000,        // Second eating sound frequency (Hz)
    GAME_OVER_FREQ_1: 400,   // Game over sound sequence
    GAME_OVER_FREQ_2: 300,
    GAME_OVER_FREQ_3: 200,
    EAT_DURATION: 0.1,       // Eating sound duration (seconds)
    GAME_OVER_DURATION: 0.2  // Game over sound duration
};
```

#### Custom Audio Profiles
```javascript
// Retro 8-bit Profile
CONFIG.AUDIO = {
    MASTER_VOLUME: 0.4,
    BASE_BPM: 140,
    MAX_BPM: 200,
    EAT_FREQ_1: 1200,        // Higher pitched
    EAT_FREQ_2: 1600,
    GAME_OVER_FREQ_1: 200,   // Lower pitched
    GAME_OVER_FREQ_2: 150,
    GAME_OVER_FREQ_3: 100
};

// Ambient Profile
CONFIG.AUDIO = {
    MASTER_VOLUME: 0.2,      // Quieter
    BASE_BPM: 80,            // Slower
    MAX_BPM: 120,
    EAT_FREQ_1: 600,         // Softer tones
    EAT_FREQ_2: 800
};
```

## 🎉 Exclamation System

### Messages and Triggers
```javascript
CONFIG.EXCLAMATIONS = {
    MESSAGES: [
        "ALL RIIIIGHT!",
        "NICE!", 
        "SNEK!!!!",
        "WOW!!!",
        "EPIC!",
        "RADICAL!",
        "GNARLY!",
        "SICK!"
    ],
    TRIGGERS: {
        SCORE_MULTIPLES: [50, 100, 200, 300, 500],  // Score thresholds
        SNAKE_LENGTHS: [10, 20, 30, 50]             // Length thresholds
    }
};
```

#### Custom Exclamation Sets
```javascript
// Classic Arcade
CONFIG.EXCLAMATIONS.MESSAGES = [
    "EXCELLENT!",
    "PERFECT!",
    "AMAZING!",
    "FANTASTIC!",
    "INCREDIBLE!",
    "OUTSTANDING!"
];

// Internet Memes
CONFIG.EXCLAMATIONS.MESSAGES = [
    "POGGERS!",
    "LET'S GOOO!",
    "SHEESH!",
    "NO CAP!",
    "BASED!",
    "W SNEK!"
];

// Retro Gaming
CONFIG.EXCLAMATIONS.MESSAGES = [
    "1UP!",
    "COMBO!",
    "MULTIKILL!",
    "GODLIKE!",
    "UNSTOPPABLE!",
    "LEGENDARY!"
];
```

### Trigger Customization
```javascript
// More Frequent Exclamations
CONFIG.EXCLAMATIONS.TRIGGERS = {
    SCORE_MULTIPLES: [25, 50, 75, 100, 150, 200],  // More frequent
    SNAKE_LENGTHS: [5, 10, 15, 20, 25, 30]
};

// Rare Exclamations (Expert Players)
CONFIG.EXCLAMATIONS.TRIGGERS = {
    SCORE_MULTIPLES: [100, 250, 500, 1000],        // Less frequent  
    SNAKE_LENGTHS: [20, 40, 60, 100]
};
```

## ✨ Particle Effects

### Particle Settings
```javascript
CONFIG.PARTICLES = {
    LIFETIME: 1000,          // Particle duration (ms)
    SPEED_MIN: 50,           // Minimum particle speed (pixels/second)
    SPEED_MAX: 150,          // Maximum particle speed
    SIZE_MIN: 2,             // Minimum particle size (pixels)
    SIZE_MAX: 6              // Maximum particle size
};
```

#### Particle Effect Styles
```javascript
// Subtle Effects
CONFIG.PARTICLES = {
    LIFETIME: 800,
    SPEED_MIN: 30,
    SPEED_MAX: 80,
    SIZE_MIN: 1,
    SIZE_MAX: 3
};
CONFIG.VISUAL.PARTICLE_COUNT = 8;

// Explosive Effects
CONFIG.PARTICLES = {
    LIFETIME: 1500,
    SPEED_MIN: 100,
    SPEED_MAX: 250,
    SIZE_MIN: 4,
    SIZE_MAX: 10
};
CONFIG.VISUAL.PARTICLE_COUNT = 25;

// Fireworks Style
CONFIG.PARTICLES = {
    LIFETIME: 2000,
    SPEED_MIN: 80,
    SPEED_MAX: 200,
    SIZE_MIN: 2,
    SIZE_MAX: 8
};
CONFIG.VISUAL.PARTICLE_COUNT = 30;
```

## 🏆 High Score System

### High Score Settings
```javascript
CONFIG.MAX_HIGH_SCORES = 10;
CONFIG.HIGH_SCORE_STORAGE_KEY = 'snekHighScores';

CONFIG.DEFAULT_HIGH_SCORES = [
    { name: 'CPU', score: 500 },
    { name: 'PRO', score: 400 },
    { name: 'ADV', score: 300 },
    { name: 'INT', score: 200 },
    { name: 'BEG', score: 100 },
    { name: 'SNK', score: 50 }
];
```

#### Custom Leaderboards
```javascript
// Competitive Leaderboard
CONFIG.DEFAULT_HIGH_SCORES = [
    { name: 'GOD', score: 2000 },
    { name: 'PRO', score: 1500 },
    { name: 'ACE', score: 1000 },
    { name: 'VET', score: 750 },
    { name: 'ADV', score: 500 },
    { name: 'INT', score: 300 },
    { name: 'NOV', score: 150 },
    { name: 'BEG', score: 75 }
];

// Beginner Friendly
CONFIG.DEFAULT_HIGH_SCORES = [
    { name: 'TOP', score: 200 },
    { name: 'GRT', score: 150 },
    { name: 'GUD', score: 100 },
    { name: 'OK_', score: 75 },
    { name: 'TRY', score: 50 },
    { name: 'NEW', score: 25 }
];
```

## 🎯 Game Balance Presets

### Complete Preset Configurations

#### "Classic" Mode
```javascript
// Authentic retro snake experience
CONFIG.VISUAL.GRID_SIZE = 25;
CONFIG.GAMEPLAY.BASE_SPEED = 250;
CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR = 0.97;
CONFIG.AUDIO.BASE_BPM = 100;
CONFIG.PARTICLES.LIFETIME = 600;
CONFIG.VISUAL.PARTICLE_COUNT = 8;
```

#### "Synthwave" Mode (Default)
```javascript
// Full future-retro experience
CONFIG.VISUAL.GRID_SIZE = 20;
CONFIG.GAMEPLAY.BASE_SPEED = 200;
CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR = 0.95;
CONFIG.AUDIO.BASE_BPM = 120;
CONFIG.PARTICLES.LIFETIME = 1000;
CONFIG.VISUAL.PARTICLE_COUNT = 15;
```

#### "Chaos" Mode
```javascript
// Maximum intensity experience
CONFIG.VISUAL.GRID_SIZE = 15;
CONFIG.GAMEPLAY.BASE_SPEED = 120;
CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR = 0.90;
CONFIG.AUDIO.BASE_BPM = 160;
CONFIG.PARTICLES.LIFETIME = 1500;
CONFIG.VISUAL.PARTICLE_COUNT = 30;
CONFIG.EXCLAMATIONS.TRIGGERS.SCORE_MULTIPLES = [20, 40, 60, 80, 100];
```

## 🔧 Runtime Configuration

### Changing Settings During Gameplay
```javascript
// Pause game and modify settings
gameState.gameRunning = false;

// Example: Switch to purple theme
CONFIG.VISUAL.COLORS.SNAKE = '#ff00ff';
CONFIG.VISUAL.COLORS.FOOD = '#ff66ff';

// Example: Increase particle intensity
CONFIG.VISUAL.PARTICLE_COUNT = 25;
CONFIG.PARTICLES.SPEED_MAX = 200;

// Resume game
gameState.gameRunning = true;
```

### Save Custom Configurations
```javascript
// Save current config to localStorage
localStorage.setItem('snekCustomConfig', JSON.stringify(CONFIG));

// Load saved config
const savedConfig = JSON.parse(localStorage.getItem('snekCustomConfig'));
if (savedConfig) {
    Object.assign(CONFIG, savedConfig);
}
```

## 🚀 Advanced Customization

### Adding New Visual Themes
1. Define color palette in `CONFIG.VISUAL.COLORS`
2. Adjust `GLOW_INTENSITY` for theme consistency  
3. Modify `PARTICLE_COUNT` to match theme intensity
4. Test contrast and readability

### Creating New Audio Profiles
1. Set appropriate `BASE_BPM` and `MAX_BPM` for mood
2. Adjust `MASTER_VOLUME` for comfort
3. Tune sound effect frequencies for harmony
4. Balance music and SFX volumes

### Custom Difficulty Curves
1. Set `BASE_SPEED` for initial challenge
2. Choose `SPEED_INCREASE_FACTOR` for progression steepness
3. Set `MIN_SPEED` for maximum difficulty cap
4. Adjust `LEVEL_THRESHOLD` for progression pacing

---

**Pro Tip**: Start with small modifications and test thoroughly. The CONFIG system makes it easy to experiment and find your perfect SNEK experience! 🐍✨