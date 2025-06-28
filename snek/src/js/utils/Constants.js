// Game constants
const CONSTANTS = {
    GRID_SIZE: 20,
    GAME_SPEED: 150,
    
    // Canvas dimensions
    CANVAS_WIDTH: 800,
    CANVAS_HEIGHT: 600,
    
    // Starting position (center of canvas)
    START_X: 20,
    START_Y: 15,
    
    // Scoring
    FOOD_POINTS: 10,
    
    // Audio frequencies
    AUDIO: {
        EAT_FREQ_1: 800,
        EAT_FREQ_2: 1000,
        GAME_OVER_FREQ_1: 400,
        GAME_OVER_FREQ_2: 300,
        GAME_OVER_FREQ_3: 200,
        EAT_DURATION: 0.1,
        GAME_OVER_DURATION: 0.2
    },
    
    // High scores
    MAX_HIGH_SCORES: 10,
    HIGH_SCORE_STORAGE_KEY: 'snakeHighScores',
    
    // Default high scores
    DEFAULT_HIGH_SCORES: [
        { name: 'CPU', score: 500 },
        { name: 'PRO', score: 400 },
        { name: 'ADV', score: 300 },
        { name: 'INT', score: 200 },
        { name: 'BEG', score: 100 },
        { name: 'NOV', score: 50 }
    ]
};