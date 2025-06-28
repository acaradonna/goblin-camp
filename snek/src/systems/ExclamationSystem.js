/**
 * SNEK Exclamation System
 * Achievement text animations and milestone celebrations
 */

import { CONFIG } from '../utils/config.js';

export class Exclamation {
  constructor(message, x, y) {
    this.message = message;
    this.x = x;
    this.y = y;
    this.startY = y;
    this.life = CONFIG.EXCLAMATIONS.DISPLAY_DURATION;
    this.maxLife = CONFIG.EXCLAMATIONS.DISPLAY_DURATION;
    this.velocity = -50; // Move upward
    this.scale = 0;
    this.targetScale = 1;
    this.alpha = 1;
  }
  
  update(deltaTime) {
    // Update position
    this.y += this.velocity * deltaTime / 1000;
    
    // Update scale (ease in)
    const scaleSpeed = 5;
    this.scale += (this.targetScale - this.scale) * scaleSpeed * deltaTime / 1000;
    
    // Update life and alpha
    this.life -= deltaTime;
    const lifePercent = this.life / this.maxLife;
    
    // Fade out in the last 25% of life
    if (lifePercent < 0.25) {
      this.alpha = lifePercent / 0.25;
    }
    
    return this.life > 0;
  }
  
  render(ctx, canvasWidth, canvasHeight) {
    if (this.alpha <= 0) return;
    
    ctx.save();
    
    // Calculate dynamic font size based on canvas size
    const baseFontSize = Math.min(canvasWidth, canvasHeight) * 0.08;
    const fontSize = baseFontSize * this.scale;
    
    ctx.font = `bold ${fontSize}px 'Orbitron', monospace`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.globalAlpha = this.alpha;
    
    // Glow effect
    ctx.shadowBlur = fontSize * 0.3;
    ctx.shadowColor = CONFIG.VISUAL.COLORS.GLOW;
    
    // Multiple glow layers for intensity
    for (let i = 0; i < 3; i++) {
      ctx.fillStyle = CONFIG.VISUAL.COLORS.SNAKE;
      ctx.fillText(this.message, this.x, this.y);
    }
    
    // Main text
    ctx.shadowBlur = 0;
    ctx.fillStyle = '#ffffff';
    ctx.fillText(this.message, this.x, this.y);
    
    ctx.restore();
  }
  
  isDead() {
    return this.life <= 0;
  }
}

export class ExclamationSystem {
  constructor() {
    this.exclamations = [];
    this.lastExclamationScore = 0;
    this.lastExclamationLength = 0;
    this.cooldownTime = 1000; // Minimum time between exclamations
    this.lastExclamationTime = 0;
  }
  
  checkTriggers(score, snakeLength, currentTime = Date.now()) {
    // Cooldown check
    if (currentTime - this.lastExclamationTime < this.cooldownTime) {
      return false;
    }
    
    let triggered = false;
    
    // Score-based triggers
    for (const threshold of CONFIG.EXCLAMATIONS.TRIGGERS.SCORE_MULTIPLES) {
      if (score >= threshold && this.lastExclamationScore < threshold) {
        this.showExclamation();
        this.lastExclamationScore = threshold;
        this.lastExclamationTime = currentTime;
        triggered = true;
        break;
      }
    }
    
    // Length-based triggers (only if score didn't trigger)
    if (!triggered) {
      for (const length of CONFIG.EXCLAMATIONS.TRIGGERS.SNAKE_LENGTHS) {
        if (snakeLength >= length && this.lastExclamationLength < length) {
          this.showExclamation();
          this.lastExclamationLength = length;
          this.lastExclamationTime = currentTime;
          triggered = true;
          break;
        }
      }
    }
    
    return triggered;
  }
  
  showExclamation(canvasWidth = 800, canvasHeight = 600, customMessage = null) {
    const message = customMessage || this.getRandomMessage();
    
    // Position in center of screen
    const x = canvasWidth / 2;
    const y = canvasHeight / 2;
    
    const exclamation = new Exclamation(message, x, y);
    this.exclamations.push(exclamation);
    
    // Limit number of simultaneous exclamations
    if (this.exclamations.length > 3) {
      this.exclamations.shift();
    }
    
    return message;
  }
  
  getRandomMessage() {
    const messages = CONFIG.EXCLAMATIONS.MESSAGES;
    return messages[Math.floor(Math.random() * messages.length)];
  }
  
  update(deltaTime) {
    // Update all exclamations and remove dead ones
    this.exclamations = this.exclamations.filter(exclamation => 
      exclamation.update(deltaTime)
    );
  }
  
  render(ctx, canvasWidth, canvasHeight) {
    // Render all exclamations
    this.exclamations.forEach(exclamation => 
      exclamation.render(ctx, canvasWidth, canvasHeight)
    );
  }
  
  clear() {
    this.exclamations = [];
  }
  
  reset() {
    this.clear();
    this.lastExclamationScore = 0;
    this.lastExclamationLength = 0;
    this.lastExclamationTime = 0;
  }
  
  // Manual trigger for special events
  triggerCustomExclamation(message, canvasWidth = 800, canvasHeight = 600) {
    return this.showExclamation(canvasWidth, canvasHeight, message);
  }
  
  // Game event handlers
  onGameStart() {
    this.reset();
    this.triggerCustomExclamation('GAME START!');
  }
  
  onGameOver(finalScore) {
    if (finalScore > 500) {
      this.triggerCustomExclamation('LEGENDARY!');
    } else if (finalScore > 300) {
      this.triggerCustomExclamation('EXCELLENT!');
    } else if (finalScore > 100) {
      this.triggerCustomExclamation('GOOD GAME!');
    } else {
      this.triggerCustomExclamation('TRY AGAIN!');
    }
  }
  
  onHighScore() {
    this.triggerCustomExclamation('NEW HIGH SCORE!');
  }
  
  getActiveExclamationCount() {
    return this.exclamations.length;
  }
}

export default ExclamationSystem;