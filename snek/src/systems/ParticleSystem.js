/**
 * SNEK Particle System
 * Physics-based particle effects for explosions and visual feedback
 */

import { CONFIG } from '../utils/config.js';

export class Particle {
  constructor(x, y, vx, vy, life, size, color = CONFIG.VISUAL.COLORS.GLOW) {
    this.x = x;
    this.y = y;
    this.vx = vx;
    this.vy = vy;
    this.life = life;
    this.maxLife = life;
    this.size = size;
    this.color = color;
    this.gravity = 0.98;
    this.friction = 0.99;
  }
  
  update(deltaTime) {
    // Update position
    this.x += this.vx * deltaTime / 1000;
    this.y += this.vy * deltaTime / 1000;
    
    // Apply physics
    this.vy += this.gravity * deltaTime / 1000;
    this.vx *= this.friction;
    this.vy *= this.friction;
    
    // Update life
    this.life -= deltaTime;
    
    return this.life > 0;
  }
  
  render(ctx) {
    const alpha = this.life / this.maxLife;
    const currentSize = this.size * alpha;
    
    ctx.save();
    ctx.globalAlpha = alpha;
    
    // Glow effect
    ctx.shadowBlur = currentSize * 2;
    ctx.shadowColor = this.color;
    
    // Draw particle
    ctx.fillStyle = this.color;
    ctx.beginPath();
    ctx.arc(this.x, this.y, currentSize, 0, Math.PI * 2);
    ctx.fill();
    
    ctx.restore();
  }
  
  isDead() {
    return this.life <= 0;
  }
}

export class ParticleSystem {
  constructor() {
    this.particles = [];
  }
  
  createExplosion(gridX, gridY, intensity = 1) {
    const x = gridX * CONFIG.VISUAL.GRID_SIZE + CONFIG.VISUAL.GRID_SIZE / 2;
    const y = gridY * CONFIG.VISUAL.GRID_SIZE + CONFIG.VISUAL.GRID_SIZE / 2;
    
    const particleCount = Math.floor(CONFIG.VISUAL.PARTICLE_COUNT * intensity);
    
    for (let i = 0; i < particleCount; i++) {
      const angle = (Math.PI * 2 * i) / particleCount + (Math.random() - 0.5) * 0.5;
      const speed = CONFIG.PARTICLES.SPEED_MIN + 
                   (CONFIG.PARTICLES.SPEED_MAX - CONFIG.PARTICLES.SPEED_MIN) * Math.random();
      
      const vx = Math.cos(angle) * speed * intensity;
      const vy = Math.sin(angle) * speed * intensity;
      
      const life = CONFIG.PARTICLES.LIFETIME * (0.5 + Math.random() * 0.5);
      const size = CONFIG.PARTICLES.SIZE_MIN + 
                  (CONFIG.PARTICLES.SIZE_MAX - CONFIG.PARTICLES.SIZE_MIN) * Math.random() * intensity;
      
      // Vary color slightly for visual interest
      const baseColor = CONFIG.VISUAL.COLORS.GLOW;
      const colorVariant = this.createColorVariant(baseColor);
      
      this.particles.push(new Particle(x, y, vx, vy, life, size, colorVariant));
    }
  }
  
  createColorVariant(baseColor) {
    // Extract RGB values from hex color
    const hex = baseColor.replace('#', '');
    const r = parseInt(hex.substr(0, 2), 16);
    const g = parseInt(hex.substr(2, 2), 16);
    const b = parseInt(hex.substr(4, 2), 16);
    const a = hex.length > 6 ? parseInt(hex.substr(6, 2), 16) / 255 : 1;
    
    // Add slight variations
    const variance = 20;
    const newR = Math.max(0, Math.min(255, r + (Math.random() - 0.5) * variance));
    const newG = Math.max(0, Math.min(255, g + (Math.random() - 0.5) * variance));
    const newB = Math.max(0, Math.min(255, b + (Math.random() - 0.5) * variance));
    
    return `rgba(${Math.floor(newR)}, ${Math.floor(newG)}, ${Math.floor(newB)}, ${a})`;
  }
  
  createTrail(x, y, color = CONFIG.VISUAL.COLORS.SNAKE) {
    // Create a smaller trail effect
    const particleCount = 3;
    
    for (let i = 0; i < particleCount; i++) {
      const angle = Math.random() * Math.PI * 2;
      const speed = 20 + Math.random() * 30;
      
      const vx = Math.cos(angle) * speed;
      const vy = Math.sin(angle) * speed;
      
      const life = 300 + Math.random() * 200;
      const size = 1 + Math.random() * 2;
      
      this.particles.push(new Particle(x, y, vx, vy, life, size, color));
    }
  }
  
  update(deltaTime) {
    // Update all particles and remove dead ones
    this.particles = this.particles.filter(particle => particle.update(deltaTime));
  }
  
  render(ctx) {
    // Render all particles
    this.particles.forEach(particle => particle.render(ctx));
  }
  
  clear() {
    this.particles = [];
  }
  
  getParticleCount() {
    return this.particles.length;
  }
  
  // Debug method to visualize particle bounds
  renderDebug(ctx) {
    if (this.particles.length === 0) return;
    
    ctx.save();
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
    ctx.lineWidth = 1;
    
    // Draw bounding box
    let minX = Infinity, minY = Infinity;
    let maxX = -Infinity, maxY = -Infinity;
    
    this.particles.forEach(particle => {
      minX = Math.min(minX, particle.x);
      minY = Math.min(minY, particle.y);
      maxX = Math.max(maxX, particle.x);
      maxY = Math.max(maxY, particle.y);
    });
    
    if (isFinite(minX)) {
      ctx.strokeRect(minX, minY, maxX - minX, maxY - minY);
    }
    
    ctx.restore();
  }
}

export default ParticleSystem;