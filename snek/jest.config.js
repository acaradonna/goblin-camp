module.exports = {
  testEnvironment: 'jsdom',
  
  // Setup files
  setupFilesAfterEnv: ['<rootDir>/tests/setup.js'],
  
  // Test patterns
  testMatch: [
    '<rootDir>/tests/**/*.test.js',
    '<rootDir>/src/**/__tests__/**/*.js',
    '<rootDir>/src/**/*.test.js'
  ],
  
  // Coverage settings
  collectCoverageFrom: [
    'src/**/*.js',
    '!src/index.js',
    '!src/test.js',
    '!**/node_modules/**',
    '!**/dist/**'
  ],
  
  coverageDirectory: 'coverage',
  
  coverageReporters: [
    'text',
    'text-summary',
    'lcov',
    'html'
  ],
  
  // Coverage thresholds
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80
    },
    './src/systems/': {
      branches: 85,
      functions: 85,
      lines: 85,
      statements: 85
    }
  },
  
  // Module name mapping for webpack aliases
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
    '^@components/(.*)$': '<rootDir>/src/components/$1',
    '^@systems/(.*)$': '<rootDir>/src/systems/$1',
    '^@utils/(.*)$': '<rootDir>/src/utils/$1',
    '^@styles/(.*)$': '<rootDir>/src/styles/$1',
    '\\.(css|less|scss|sass)$': 'identity-obj-proxy'
  },
  
  // Transform files
  transform: {
    '^.+\\.js$': 'babel-jest'
  },
  
  // Ignore patterns
  testPathIgnorePatterns: [
    '/node_modules/',
    '/dist/'
  ],
  
  // Globals for game testing
  globals: {
    CONFIG: {
      VISUAL: {
        GRID_SIZE: 20,
        CANVAS_WIDTH: 800,
        CANVAS_HEIGHT: 600,
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
        BASE_BPM: 120,
        MAX_BPM: 180
      }
    }
  },
  
  // Verbose output for debugging
  verbose: false,
  
  // Clear mocks between tests
  clearMocks: true,
  
  // Restore mocks after each test
  restoreMocks: true,
  
  // Error on deprecated features
  errorOnDeprecated: true,
  
  // Timeout for tests
  testTimeout: 10000
};