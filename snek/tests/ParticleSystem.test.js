/**
 * Particle System Tests
 * Tests for physics-based particle effects
 */

import ParticleSystem, { Particle } from '../src/systems/ParticleSystem.js';
import { CONFIG } from '../src/utils/config.js';

describe('Particle System', () => {
  let particleSystem;
  
  beforeEach(() => {
    particleSystem = new ParticleSystem();
  });
  
  describe('Particle Class', () => {
    test('should create particle with correct properties', () => {
      const particle = new Particle(100, 200, 50, -50, 1000, 5);
      
      expect(particle.x).toBe(100);
      expect(particle.y).toBe(200);
      expect(particle.vx).toBe(50);
      expect(particle.vy).toBe(-50);
      expect(particle.life).toBe(1000);
      expect(particle.maxLife).toBe(1000);
      expect(particle.size).toBe(5);
      expect(particle.gravity).toBe(0.98);
      expect(particle.friction).toBe(0.99);
    });
    
    test('should update particle position and physics', () => {
      const particle = new Particle(0, 0, 100, 0, 1000, 5);
      const initialX = particle.x;
      const initialY = particle.y;
      const initialVY = particle.vy;
      const initialLife = particle.life;
      
      const isAlive = particle.update(100); // 100ms
      
      expect(particle.x).not.toBe(initialX);
      // Gravity affects velocity, which affects position on subsequent updates
      expect(particle.vy).toBeGreaterThan(initialVY);
      expect(particle.life).toBeLessThan(initialLife);
      expect(isAlive).toBe(true);
    });
    
    test('should apply gravity and friction', () => {
      const particle = new Particle(0, 0, 100, 0, 1000, 5);
      const initialVX = particle.vx;
      const initialVY = particle.vy;
      
      particle.update(100);
      
      expect(particle.vx).toBeLessThan(initialVX); // Friction applied
      expect(particle.vy).toBeGreaterThan(initialVY); // Gravity applied
    });
    
    test('should die when life reaches zero', () => {
      const particle = new Particle(0, 0, 0, 0, 50, 5);
      
      const isAlive = particle.update(100); // More than particle life
      
      expect(isAlive).toBe(false);
      expect(particle.isDead()).toBe(true);
    });
    
    test('should render without throwing errors', () => {
      const particle = new Particle(0, 0, 0, 0, 1000, 5);
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        beginPath: jest.fn(),
        arc: jest.fn(),
        fill: jest.fn(),
        set globalAlpha(value) {},
        set shadowBlur(value) {},
        set shadowColor(value) {},
        set fillStyle(value) {}
      };
      
      expect(() => particle.render(mockCtx)).not.toThrow();
      expect(mockCtx.save).toHaveBeenCalled();
      expect(mockCtx.restore).toHaveBeenCalled();
    });
  });
  
  describe('ParticleSystem Class', () => {
    test('should initialize with empty particle array', () => {
      expect(particleSystem.particles).toEqual([]);
      expect(particleSystem.getParticleCount()).toBe(0);
    });
    
    test('should create explosion particles correctly', () => {
      const x = 10, y = 10, intensity = 1;
      particleSystem.createExplosion(x, y, intensity);
      
      const expectedCount = Math.floor(CONFIG.VISUAL.PARTICLE_COUNT * intensity);
      expect(particleSystem.particles.length).toBe(expectedCount);
      
      particleSystem.particles.forEach(particle => {
        expect(typeof particle.x).toBe('number');
        expect(typeof particle.y).toBe('number');
        expect(typeof particle.vx).toBe('number');
        expect(typeof particle.vy).toBe('number');
        expect(typeof particle.life).toBe('number');
        expect(typeof particle.size).toBe('number');
        
        expect(particle.life).toBeGreaterThan(0);
        expect(particle.size).toBeWithinRange(CONFIG.PARTICLES.SIZE_MIN, CONFIG.PARTICLES.SIZE_MAX);
      });
    });
    
    test('should scale particle count with intensity', () => {
      const lowIntensity = 0.5;
      particleSystem.createExplosion(5, 5, lowIntensity);
      const lowCount = particleSystem.particles.length;
      
      particleSystem.clear();
      
      const highIntensity = 2.0;
      particleSystem.createExplosion(5, 5, highIntensity);
      const highCount = particleSystem.particles.length;
      
      expect(highCount).toBeGreaterThan(lowCount);
    });
    
    test('should position particles correctly in grid coordinates', () => {
      const gridX = 5, gridY = 10;
      particleSystem.createExplosion(gridX, gridY, 1);
      
      const expectedX = gridX * CONFIG.VISUAL.GRID_SIZE + CONFIG.VISUAL.GRID_SIZE / 2;
      const expectedY = gridY * CONFIG.VISUAL.GRID_SIZE + CONFIG.VISUAL.GRID_SIZE / 2;
      
      particleSystem.particles.forEach(particle => {
        expect(particle.x).toBe(expectedX);
        expect(particle.y).toBe(expectedY);
      });
    });
    
    test('should update all particles and remove dead ones', () => {
      particleSystem.createExplosion(10, 10, 1);
      const initialCount = particleSystem.particles.length;
      const initialLife = particleSystem.particles[0].life;
      
      // Update with small delta time
      particleSystem.update(100);
      
      expect(particleSystem.particles.length).toBeLessThanOrEqual(initialCount);
      if (particleSystem.particles.length > 0) {
        expect(particleSystem.particles[0].life).toBeLessThan(initialLife);
      }
    });
    
    test('should remove all dead particles', () => {
      particleSystem.createExplosion(10, 10, 1);
      const initialCount = particleSystem.particles.length;
      
      // Update with large delta time to kill all particles
      particleSystem.update(CONFIG.PARTICLES.LIFETIME + 100);
      
      expect(particleSystem.particles.length).toBe(0);
    });
    
    test('should create trail particles', () => {
      const x = 100, y = 200;
      particleSystem.createTrail(x, y);
      
      expect(particleSystem.particles.length).toBe(3); // Trail creates 3 particles
      
      particleSystem.particles.forEach(particle => {
        expect(particle.x).toBe(x);
        expect(particle.y).toBe(y);
        expect(particle.life).toBeLessThan(600); // Trail particles have shorter life
      });
    });
    
    test('should render all particles without errors', () => {
      particleSystem.createExplosion(10, 10, 1);
      
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        beginPath: jest.fn(),
        arc: jest.fn(),
        fill: jest.fn(),
        set globalAlpha(value) {},
        set shadowBlur(value) {},
        set shadowColor(value) {},
        set fillStyle(value) {}
      };
      
      expect(() => particleSystem.render(mockCtx)).not.toThrow();
    });
    
    test('should clear all particles', () => {
      particleSystem.createExplosion(10, 10, 1);
      expect(particleSystem.particles.length).toBeGreaterThan(0);
      
      particleSystem.clear();
      expect(particleSystem.particles.length).toBe(0);
    });
    
    test('should create color variants', () => {
      const baseColor = '#00ff41';
      const variant1 = particleSystem.createColorVariant(baseColor);
      const variant2 = particleSystem.createColorVariant(baseColor);
      
      expect(typeof variant1).toBe('string');
      expect(typeof variant2).toBe('string');
      expect(variant1).toMatch(/rgba\(\d+,\s*\d+,\s*\d+,\s*[\d.]+\)/);
    });
    
    test('should render debug information', () => {
      particleSystem.createExplosion(10, 10, 1);
      
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        strokeRect: jest.fn(),
        set strokeStyle(value) {},
        set lineWidth(value) {}
      };
      
      expect(() => particleSystem.renderDebug(mockCtx)).not.toThrow();
      expect(mockCtx.save).toHaveBeenCalled();
      expect(mockCtx.restore).toHaveBeenCalled();
    });
    
    test('should handle empty particle array in debug render', () => {
      const mockCtx = {
        save: jest.fn(),
        restore: jest.fn(),
        strokeRect: jest.fn(),
        set strokeStyle(value) {},
        set lineWidth(value) {}
      };
      
      expect(() => particleSystem.renderDebug(mockCtx)).not.toThrow();
      expect(mockCtx.strokeRect).not.toHaveBeenCalled();
    });
  });
});