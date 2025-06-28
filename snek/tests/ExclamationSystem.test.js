/**
 * Exclamation System Tests
 * Tests for achievement text animations and milestone celebrations
 */

import ExclamationSystem, { Exclamation } from '../src/systems/ExclamationSystem.js';
import { CONFIG } from '../src/utils/config.js';

describe('Exclamation System', () => {
  let exclamationSystem;
  
  beforeEach(() => {
    exclamationSystem = new ExclamationSystem();
  });
  
  describe('Exclamation Class', () => {
    test('should create exclamation with correct properties', () => {
      const message = 'TEST!';
      const x = 400, y = 300;
      const exclamation = new Exclamation(message, x, y);
      
      expect(exclamation.message).toBe(message);
      expect(exclamation.x).toBe(x);
      expect(exclamation.y).toBe(y);
      expect(exclamation.startY).toBe(y);
      expect(exclamation.life).toBe(CONFIG.EXCLAMATIONS.DISPLAY_DURATION);
      expect(exclamation.maxLife).toBe(CONFIG.EXCLAMATIONS.DISPLAY_DURATION);
      expect(exclamation.velocity).toBe(-50);
      expect(exclamation.scale).toBe(0);
      expect(exclamation.targetScale).toBe(1);
      expect(exclamation.alpha).toBe(1);
    });
    
    test('should update position and properties over time', () => {
      const exclamation = new Exclamation('TEST!', 400, 300);
      const initialY = exclamation.y;
      const initialLife = exclamation.life;
      const initialScale = exclamation.scale;
      
      const isAlive = exclamation.update(100); // 100ms
      
      expect(exclamation.y).toBeLessThan(initialY); // Moves upward
      expect(exclamation.life).toBeLessThan(initialLife);
      expect(exclamation.scale).toBeGreaterThan(initialScale); // Scale increases
      expect(isAlive).toBe(true);
    });
    
    test('should fade out near end of life', () => {
      const exclamation = new Exclamation('TEST!', 400, 300);
      // Set life to 20% of max (in fade zone)
      exclamation.life = CONFIG.EXCLAMATIONS.DISPLAY_DURATION * 0.2;
      
      exclamation.update(100);
      
      expect(exclamation.alpha).toBeLessThan(1);
    });
    
    test('should die when life reaches zero', () => {
      const exclamation = new Exclamation('TEST!', 400, 300);
      
      const isAlive = exclamation.update(CONFIG.EXCLAMATIONS.DISPLAY_DURATION + 100);
      
      expect(isAlive).toBe(false);
      expect(exclamation.isDead()).toBe(true);
    });
    
    test('should render without throwing errors', () => {
      const exclamation = new Exclamation('TEST!', 400, 300);
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        fillText: jest.fn(),
        set font(value) {},
        set textAlign(value) {},
        set textBaseline(value) {},
        set globalAlpha(value) {},
        set shadowBlur(value) {},
        set shadowColor(value) {},
        set fillStyle(value) {}
      };
      
      expect(() => exclamation.render(mockCtx, 800, 600)).not.toThrow();
      expect(mockCtx.save).toHaveBeenCalled();
      expect(mockCtx.restore).toHaveBeenCalled();
      expect(mockCtx.fillText).toHaveBeenCalled();
    });
    
    test('should not render when alpha is zero', () => {
      const exclamation = new Exclamation('TEST!', 400, 300);
      exclamation.alpha = 0;
      
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        fillText: jest.fn()
      };
      
      exclamation.render(mockCtx, 800, 600);
      
      expect(mockCtx.fillText).not.toHaveBeenCalled();
    });
  });
  
  describe('ExclamationSystem Class', () => {
    test('should initialize with correct default values', () => {
      expect(exclamationSystem.exclamations).toEqual([]);
      expect(exclamationSystem.lastExclamationScore).toBe(0);
      expect(exclamationSystem.lastExclamationLength).toBe(0);
      expect(exclamationSystem.cooldownTime).toBe(1000);
      expect(exclamationSystem.lastExclamationTime).toBe(0);
      expect(exclamationSystem.getActiveExclamationCount()).toBe(0);
    });
    
    test('should show exclamation with valid message', () => {
      const message = exclamationSystem.showExclamation();
      
      expect(typeof message).toBe('string');
      expect(CONFIG.EXCLAMATIONS.MESSAGES.includes(message)).toBe(true);
      expect(exclamationSystem.exclamations.length).toBe(1);
    });
    
    test('should show custom exclamation message', () => {
      const customMessage = 'CUSTOM MESSAGE!';
      const message = exclamationSystem.showExclamation(800, 600, customMessage);
      
      expect(message).toBe(customMessage);
      expect(exclamationSystem.exclamations[0].message).toBe(customMessage);
    });
    
    test('should limit simultaneous exclamations to 3', () => {
      for (let i = 0; i < 5; i++) {
        exclamationSystem.showExclamation();
      }
      
      expect(exclamationSystem.exclamations.length).toBe(3);
    });
    
    test('should trigger on score milestones', () => {
      exclamationSystem.lastExclamationScore = 0;
      exclamationSystem.lastExclamationLength = 0;
      exclamationSystem.lastExclamationTime = 0;
      
      const triggered = exclamationSystem.checkTriggers(50, 5, 2000); // First score threshold, past cooldown
      
      expect(triggered).toBe(true);
      expect(exclamationSystem.lastExclamationScore).toBe(50);
      expect(exclamationSystem.exclamations.length).toBe(1);
    });
    
    test('should trigger on snake length milestones', () => {
      exclamationSystem.lastExclamationScore = 0;
      exclamationSystem.lastExclamationLength = 0;
      exclamationSystem.lastExclamationTime = 0;
      
      const triggered = exclamationSystem.checkTriggers(25, 10, 2000); // First length threshold, past cooldown
      
      expect(triggered).toBe(true);
      expect(exclamationSystem.lastExclamationLength).toBe(10);
      expect(exclamationSystem.exclamations.length).toBe(1);
    });
    
    test('should not trigger multiple times for same milestone', () => {
      exclamationSystem.lastExclamationScore = 50;
      exclamationSystem.lastExclamationLength = 10;
      
      const triggered = exclamationSystem.checkTriggers(50, 10, 0);
      
      expect(triggered).toBe(false);
      expect(exclamationSystem.exclamations.length).toBe(0);
    });
    
    test('should respect cooldown period', () => {
      exclamationSystem.lastExclamationTime = 500;
      
      const triggered = exclamationSystem.checkTriggers(100, 5, 1000); // 500ms after last
      
      expect(triggered).toBe(false);
    });
    
    test('should prioritize score triggers over length triggers', () => {
      exclamationSystem.lastExclamationScore = 0;
      exclamationSystem.lastExclamationLength = 0;
      exclamationSystem.lastExclamationTime = 0;
      
      // Both score and length thresholds met
      const triggered = exclamationSystem.checkTriggers(50, 10, 2000); // Past cooldown
      
      expect(triggered).toBe(true);
      expect(exclamationSystem.lastExclamationScore).toBe(50);
      expect(exclamationSystem.lastExclamationLength).toBe(0); // Should not update length
    });
    
    test('should update exclamations and remove dead ones', () => {
      exclamationSystem.showExclamation();
      const initialCount = exclamationSystem.exclamations.length;
      
      // Update with small delta time
      exclamationSystem.update(100);
      expect(exclamationSystem.exclamations.length).toBe(initialCount);
      
      // Update with large delta time to kill exclamations
      exclamationSystem.update(CONFIG.EXCLAMATIONS.DISPLAY_DURATION + 100);
      expect(exclamationSystem.exclamations.length).toBe(0);
    });
    
    test('should render all exclamations without errors', () => {
      exclamationSystem.showExclamation();
      
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        fillText: jest.fn(),
        set font(value) {},
        set textAlign(value) {},
        set textBaseline(value) {},
        set globalAlpha(value) {},
        set shadowBlur(value) {},
        set shadowColor(value) {},
        set fillStyle(value) {}
      };
      
      expect(() => exclamationSystem.render(mockCtx, 800, 600)).not.toThrow();
    });
    
    test('should clear all exclamations', () => {
      exclamationSystem.showExclamation();
      expect(exclamationSystem.exclamations.length).toBeGreaterThan(0);
      
      exclamationSystem.clear();
      expect(exclamationSystem.exclamations.length).toBe(0);
    });
    
    test('should reset system state', () => {
      exclamationSystem.showExclamation();
      exclamationSystem.lastExclamationScore = 100;
      exclamationSystem.lastExclamationLength = 20;
      exclamationSystem.lastExclamationTime = 5000;
      
      exclamationSystem.reset();
      
      expect(exclamationSystem.exclamations.length).toBe(0);
      expect(exclamationSystem.lastExclamationScore).toBe(0);
      expect(exclamationSystem.lastExclamationLength).toBe(0);
      expect(exclamationSystem.lastExclamationTime).toBe(0);
    });
    
    test('should get random message from config', () => {
      const message = exclamationSystem.getRandomMessage();
      
      expect(typeof message).toBe('string');
      expect(CONFIG.EXCLAMATIONS.MESSAGES.includes(message)).toBe(true);
    });
    
    test('should trigger custom exclamation manually', () => {
      const customMessage = 'MANUAL TRIGGER!';
      const message = exclamationSystem.triggerCustomExclamation(customMessage);
      
      expect(message).toBe(customMessage);
      expect(exclamationSystem.exclamations.length).toBe(1);
    });
    
    describe('Game Event Handlers', () => {
      test('should handle game start event', () => {
        exclamationSystem.onGameStart();
        
        expect(exclamationSystem.exclamations.length).toBe(1);
        expect(exclamationSystem.exclamations[0].message).toBe('GAME START!');
      });
      
      test('should handle game over with legendary score', () => {
        exclamationSystem.onGameOver(600);
        
        expect(exclamationSystem.exclamations.length).toBe(1);
        expect(exclamationSystem.exclamations[0].message).toBe('LEGENDARY!');
      });
      
      test('should handle game over with excellent score', () => {
        exclamationSystem.onGameOver(400);
        
        expect(exclamationSystem.exclamations.length).toBe(1);
        expect(exclamationSystem.exclamations[0].message).toBe('EXCELLENT!');
      });
      
      test('should handle game over with good score', () => {
        exclamationSystem.onGameOver(200);
        
        expect(exclamationSystem.exclamations.length).toBe(1);
        expect(exclamationSystem.exclamations[0].message).toBe('GOOD GAME!');
      });
      
      test('should handle game over with low score', () => {
        exclamationSystem.onGameOver(50);
        
        expect(exclamationSystem.exclamations.length).toBe(1);
        expect(exclamationSystem.exclamations[0].message).toBe('TRY AGAIN!');
      });
      
      test('should handle high score event', () => {
        exclamationSystem.onHighScore();
        
        expect(exclamationSystem.exclamations.length).toBe(1);
        expect(exclamationSystem.exclamations[0].message).toBe('NEW HIGH SCORE!');
      });
    });
  });
});