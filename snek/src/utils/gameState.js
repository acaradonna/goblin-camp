/**
 * SNEK Game State Management
 * Centralized game state with proper initialization and validation
 */

import { CONFIG } from './config.js';

export const createInitialGameState = () => ({
  currentScreen: 'menu',
  score: 0,
  level: 1,
  speed: CONFIG.GAMEPLAY.BASE_SPEED,
  snake: [{ x: 20, y: 15 }],
  food: null,
  foodEaten: 0,
  direction: { x: 0, y: 0 },
  nextDirection: { x: 0, y: 0 },
  gameRunning: false,
  isPaused: false,
  lastTime: 0,
  accumulator: 0,
  previousSnake: [],
  previousFood: null,
  gameStartTime: null,
  gameEndTime: null
});

export let gameState = createInitialGameState();

export const resetGameState = () => {
  Object.assign(gameState, createInitialGameState());
  return gameState;
};

export const updateGameState = (updates) => {
  Object.assign(gameState, updates);
  return gameState;
};

export const validateGameState = (state = gameState) => {
  const errors = [];
  
  if (!state.snake || !Array.isArray(state.snake) || state.snake.length === 0) {
    errors.push('Invalid snake array');
  }
  
  if (typeof state.score !== 'number' || state.score < 0) {
    errors.push('Invalid score');
  }
  
  if (typeof state.level !== 'number' || state.level < 1) {
    errors.push('Invalid level');
  }
  
  if (typeof state.speed !== 'number' || state.speed < CONFIG.GAMEPLAY.MIN_SPEED) {
    errors.push('Invalid speed');
  }
  
  if (!['menu', 'playing', 'gameOver', 'paused'].includes(state.currentScreen)) {
    errors.push('Invalid screen state');
  }
  
  return {
    isValid: errors.length === 0,
    errors
  };
};

export default gameState;