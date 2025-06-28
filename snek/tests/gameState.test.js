/**
 * Game State Management Tests
 * Tests for game state initialization and validation
 */

import { createInitialGameState, resetGameState, updateGameState, validateGameState } from '../src/utils/gameState.js';
import { CONFIG } from '../src/utils/config.js';

describe('Game State Management', () => {
  
  test('should create valid initial game state', () => {
    const state = createInitialGameState();
    
    expect(state.currentScreen).toBe('menu');
    expect(state.score).toBe(0);
    expect(state.level).toBe(1);
    expect(state.speed).toBe(CONFIG.GAMEPLAY.BASE_SPEED);
    expect(Array.isArray(state.snake)).toBe(true);
    expect(state.snake).toEqual([{ x: 20, y: 15 }]);
    expect(state.foodEaten).toBe(0);
    expect(state.gameRunning).toBe(false);
  });
  
  test('should reset game state correctly', () => {
    const state = resetGameState();
    
    expect(state.currentScreen).toBe('menu');
    expect(state.score).toBe(0);
    expect(state.level).toBe(1);
    expect(state.snake.length).toBe(1);
    expect(state.gameRunning).toBe(false);
  });
  
  test('should update game state', () => {
    const updates = {
      score: 100,
      level: 2,
      gameRunning: true
    };
    
    const state = updateGameState(updates);
    
    expect(state.score).toBe(100);
    expect(state.level).toBe(2);
    expect(state.gameRunning).toBe(true);
  });
  
  test('should validate correct game state', () => {
    const validState = createInitialGameState();
    const validation = validateGameState(validState);
    
    expect(validation.isValid).toBe(true);
    expect(validation.errors).toHaveLength(0);
  });
  
  test('should detect invalid snake array', () => {
    const invalidState = createInitialGameState();
    invalidState.snake = [];
    
    const validation = validateGameState(invalidState);
    
    expect(validation.isValid).toBe(false);
    expect(validation.errors).toContain('Invalid snake array');
  });
  
  test('should detect invalid score', () => {
    const invalidState = createInitialGameState();
    invalidState.score = -10;
    
    const validation = validateGameState(invalidState);
    
    expect(validation.isValid).toBe(false);
    expect(validation.errors).toContain('Invalid score');
  });
  
  test('should detect invalid level', () => {
    const invalidState = createInitialGameState();
    invalidState.level = 0;
    
    const validation = validateGameState(invalidState);
    
    expect(validation.isValid).toBe(false);
    expect(validation.errors).toContain('Invalid level');
  });
  
  test('should detect invalid speed', () => {
    const invalidState = createInitialGameState();
    invalidState.speed = CONFIG.GAMEPLAY.MIN_SPEED - 10;
    
    const validation = validateGameState(invalidState);
    
    expect(validation.isValid).toBe(false);
    expect(validation.errors).toContain('Invalid speed');
  });
  
  test('should detect invalid screen state', () => {
    const invalidState = createInitialGameState();
    invalidState.currentScreen = 'invalid';
    
    const validation = validateGameState(invalidState);
    
    expect(validation.isValid).toBe(false);
    expect(validation.errors).toContain('Invalid screen state');
  });
  
});