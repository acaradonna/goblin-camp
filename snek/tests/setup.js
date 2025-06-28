/**
 * Jest setup file for SNEK game testing
 * This file is executed before each test file
 */

// Mock Web Audio API for testing
global.AudioContext = class MockAudioContext {
  constructor() {
    this.currentTime = 0;
    this.destination = {};
    this.sampleRate = 44100;
    this.state = 'running';
  }
  
  createOscillator() {
    return {
      frequency: { setValueAtTime: jest.fn() },
      type: 'sine',
      connect: jest.fn(),
      start: jest.fn(),
      stop: jest.fn()
    };
  }
  
  createGain() {
    return {
      gain: {
        setValueAtTime: jest.fn(),
        linearRampToValueAtTime: jest.fn(),
        exponentialRampToValueAtTime: jest.fn()
      },
      connect: jest.fn()
    };
  }
  
  createDynamicsCompressor() {
    return { connect: jest.fn() };
  }
  
  createConvolver() {
    return { 
      buffer: null,
      connect: jest.fn() 
    };
  }
  
  createBiquadFilter() {
    return {
      type: 'lowpass',
      frequency: { setValueAtTime: jest.fn() },
      Q: { setValueAtTime: jest.fn() },
      connect: jest.fn()
    };
  }
  
  createBuffer(channels, length, sampleRate) {
    return {
      numberOfChannels: channels,
      length: length,
      sampleRate: sampleRate,
      getChannelData: jest.fn(() => new Float32Array(length))
    };
  }
};

// Mock HTML5 Canvas API for testing
global.HTMLCanvasElement.prototype.getContext = jest.fn(() => ({
  fillStyle: '',
  fillRect: jest.fn(),
  beginPath: jest.fn(),
  arc: jest.fn(),
  fill: jest.fn(),
  save: jest.fn(),
  restore: jest.fn(),
  globalAlpha: 1,
  shadowBlur: 0,
  shadowColor: '',
  clearRect: jest.fn(),
  strokeStyle: '',
  stroke: jest.fn(),
  lineWidth: 1,
  moveTo: jest.fn(),
  lineTo: jest.fn(),
  setTransform: jest.fn(),
  translate: jest.fn(),
  rotate: jest.fn(),
  scale: jest.fn()
}));

// Mock requestAnimationFrame for testing
global.requestAnimationFrame = jest.fn(cb => setTimeout(cb, 16));
global.cancelAnimationFrame = jest.fn(id => clearTimeout(id));

// Mock performance.now() for testing
global.performance = {
  now: jest.fn(() => Date.now())
};

// Mock localStorage for testing
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn()
};
global.localStorage = localStorageMock;

// Mock window object properties
Object.defineProperty(window, 'innerWidth', {
  writable: true,
  configurable: true,
  value: 1024
});

Object.defineProperty(window, 'innerHeight', {
  writable: true,
  configurable: true,
  value: 768
});

// Console setup for tests
global.console = {
  ...console,
  // Uncomment to suppress console.log during tests
  // log: jest.fn(),
  // warn: jest.fn(),
  // error: jest.fn()
};

// Custom matchers for game testing
expect.extend({
  toBeValidCoordinates(received) {
    const pass = 
      received &&
      typeof received.x === 'number' &&
      typeof received.y === 'number' &&
      received.x >= 0 &&
      received.y >= 0;
    
    if (pass) {
      return {
        message: () => `expected ${JSON.stringify(received)} not to be valid coordinates`,
        pass: true
      };
    } else {
      return {
        message: () => `expected ${JSON.stringify(received)} to be valid coordinates`,
        pass: false
      };
    }
  },
  
  toBeWithinRange(received, min, max) {
    const pass = received >= min && received <= max;
    
    if (pass) {
      return {
        message: () => `expected ${received} not to be within range ${min}-${max}`,
        pass: true
      };
    } else {
      return {
        message: () => `expected ${received} to be within range ${min}-${max}`,
        pass: false
      };
    }
  }
});

// Global test utilities
global.testUtils = {
  createMockGameState: () => ({
    currentScreen: 'menu',
    score: 0,
    level: 1,
    speed: 200,
    snake: [{ x: 20, y: 15 }],
    food: null,
    foodEaten: 0,
    direction: { x: 0, y: 0 },
    nextDirection: { x: 0, y: 0 },
    gameRunning: false,
    lastTime: 0,
    accumulator: 0
  }),
  
  wait: (ms) => new Promise(resolve => setTimeout(resolve, ms)),
  
  triggerKeyPress: (key) => {
    const event = new KeyboardEvent('keydown', { key });
    document.dispatchEvent(event);
    return event;
  }
};