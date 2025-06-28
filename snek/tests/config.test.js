/**
 * Configuration System Tests
 * Tests for the centralized CONFIG object
 */

import { CONFIG } from '../src/utils/config.js';

describe('Configuration System', () => {
  
  test('should have valid visual configuration', () => {
    expect(typeof CONFIG.VISUAL.GRID_SIZE).toBe('number');
    expect(CONFIG.VISUAL.GRID_SIZE).toBeGreaterThan(0);
    
    expect(typeof CONFIG.VISUAL.CANVAS_WIDTH).toBe('number');
    expect(typeof CONFIG.VISUAL.CANVAS_HEIGHT).toBe('number');
    expect(CONFIG.VISUAL.CANVAS_WIDTH).toBeGreaterThan(0);
    expect(CONFIG.VISUAL.CANVAS_HEIGHT).toBeGreaterThan(0);
    
    expect(typeof CONFIG.VISUAL.PARTICLE_COUNT).toBe('number');
    expect(CONFIG.VISUAL.PARTICLE_COUNT).toBeGreaterThan(0);
  });
  
  test('should have valid color configuration', () => {
    const requiredColors = ['SNAKE', 'FOOD', 'BACKGROUND', 'GRID', 'GLOW'];
    requiredColors.forEach(color => {
      expect(typeof CONFIG.VISUAL.COLORS[color]).toBe('string');
      expect(CONFIG.VISUAL.COLORS[color].length).toBeGreaterThan(0);
    });
  });
  
  test('should have valid gameplay configuration', () => {
    expect(typeof CONFIG.GAMEPLAY.BASE_SPEED).toBe('number');
    expect(CONFIG.GAMEPLAY.BASE_SPEED).toBeGreaterThan(0);
    
    expect(typeof CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR).toBe('number');
    expect(CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR).toBeGreaterThan(0);
    expect(CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR).toBeLessThan(1);
    
    expect(typeof CONFIG.GAMEPLAY.MIN_SPEED).toBe('number');
    expect(CONFIG.GAMEPLAY.MIN_SPEED).toBeGreaterThan(0);
    expect(CONFIG.GAMEPLAY.MIN_SPEED).toBeLessThan(CONFIG.GAMEPLAY.BASE_SPEED);
  });
  
  test('should have valid audio configuration', () => {
    expect(typeof CONFIG.AUDIO.MASTER_VOLUME).toBe('number');
    expect(CONFIG.AUDIO.MASTER_VOLUME).toBeWithinRange(0, 1);
    
    expect(typeof CONFIG.AUDIO.BASE_BPM).toBe('number');
    expect(CONFIG.AUDIO.BASE_BPM).toBeGreaterThan(0);
    
    expect(typeof CONFIG.AUDIO.MAX_BPM).toBe('number');
    expect(CONFIG.AUDIO.MAX_BPM).toBeGreaterThan(CONFIG.AUDIO.BASE_BPM);
  });
  
  test('should have valid exclamation configuration', () => {
    expect(Array.isArray(CONFIG.EXCLAMATIONS.MESSAGES)).toBe(true);
    expect(CONFIG.EXCLAMATIONS.MESSAGES.length).toBeGreaterThan(0);
    
    CONFIG.EXCLAMATIONS.MESSAGES.forEach(message => {
      expect(typeof message).toBe('string');
      expect(message.length).toBeGreaterThan(0);
    });
    
    expect(Array.isArray(CONFIG.EXCLAMATIONS.TRIGGERS.SCORE_MULTIPLES)).toBe(true);
    expect(Array.isArray(CONFIG.EXCLAMATIONS.TRIGGERS.SNAKE_LENGTHS)).toBe(true);
  });
  
  test('should have valid particle configuration', () => {
    expect(typeof CONFIG.PARTICLES.LIFETIME).toBe('number');
    expect(CONFIG.PARTICLES.LIFETIME).toBeGreaterThan(0);
    
    expect(typeof CONFIG.PARTICLES.SPEED_MIN).toBe('number');
    expect(typeof CONFIG.PARTICLES.SPEED_MAX).toBe('number');
    expect(CONFIG.PARTICLES.SPEED_MIN).toBeLessThan(CONFIG.PARTICLES.SPEED_MAX);
    
    expect(typeof CONFIG.PARTICLES.SIZE_MIN).toBe('number');
    expect(typeof CONFIG.PARTICLES.SIZE_MAX).toBe('number');
    expect(CONFIG.PARTICLES.SIZE_MIN).toBeLessThan(CONFIG.PARTICLES.SIZE_MAX);
  });

  test('should have valid controls configuration', () => {
    const requiredControls = ['UP', 'DOWN', 'LEFT', 'RIGHT', 'PAUSE', 'MUTE', 'RESTART'];
    requiredControls.forEach(control => {
      expect(Array.isArray(CONFIG.CONTROLS[control])).toBe(true);
      expect(CONFIG.CONTROLS[control].length).toBeGreaterThan(0);
    });
  });
  
});