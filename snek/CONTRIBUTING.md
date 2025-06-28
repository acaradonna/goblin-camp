# Contributing to SNEK 🐍

Welcome to the SNEK community! We're excited to have you contribute to this future-retro arcade experience. This guide will help you get started with contributing to the project.

## 🚀 Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a feature branch** from `main`
4. **Make your changes** following our guidelines
5. **Test thoroughly** across different browsers
6. **Submit a pull request** with a clear description

## 📋 Prerequisites

- Modern web browser (Chrome 60+, Firefox 55+, Safari 11+, Edge 79+)
- Basic knowledge of JavaScript ES6+, HTML5 Canvas, and CSS3
- Understanding of Web Audio API for audio-related contributions
- Text editor or IDE of your choice

## 🏗️ Project Structure

```
snek/
├── snek.html              # Main game file (current architecture)
├── snake.html             # Legacy version
├── src/                   # Modular architecture (in development)
│   ├── js/
│   │   ├── core/         # Game engine components
│   │   ├── entities/     # Game objects (Snake, Food)
│   │   ├── systems/      # Game systems (Audio, Input, Score)
│   │   └── utils/        # Utilities and constants
│   ├── css/              # Modular stylesheets
│   └── index.html        # Clean HTML structure
├── README.md
├── REFACTOR-PLAN.md
└── CONTRIBUTING.md
```

## 🎨 Code Style Guidelines

### JavaScript

```javascript
// Use ES6+ features
const CONFIG = {
    VISUAL: {
        GRID_SIZE: 20,
        COLORS: {
            SNAKE: '#00ff41',
            FOOD: '#66ff66'
        }
    }
};

// Use descriptive function names
function createSynthVoice(frequency, startTime, duration, type, channel, options = {}) {
    // Implementation
}

// Use camelCase for variables and functions
let gameState = { currentScreen: 'menu' };
function updateGameSpeed() { /* ... */ }

// Use PascalCase for classes
class AudioSystem {
    constructor() {
        this.context = null;
    }
}
```

### CSS

```css
/* Use CSS custom properties for theming */
:root {
    --neon-green: #00ff41;
    --dark-green: #003d10;
    --glow-green: #00ff4180;
}

/* Use semantic class names */
.game-container {
    text-align: center;
}

.synthwave-button {
    background: transparent;
    border: 2px solid var(--neon-green);
}
```

### HTML

```html
<!-- Use semantic HTML -->
<main class="game-container">
    <section class="hud">
        <div class="score-display">Score: <span id="score">0</span></div>
    </section>
</main>

<!-- Use descriptive IDs -->
<canvas id="gameCanvas" width="800" height="600"></canvas>
<button id="muteBtn" class="mute-btn">🔊</button>
```

## 🎵 Audio System Guidelines

When contributing to the synthwave audio system:

### Frequency Standards
```javascript
// Use standard musical frequencies
const NOTES = {
    C2: 65.41,
    C3: 130.81,
    C4: 261.63,  // Middle C
    C5: 523.25
};

// Follow the existing ADSR envelope pattern
const envelope = {
    attack: 0.01,   // Fast attack for punchy sounds
    decay: 0.1,     // Quick decay
    sustain: 0.7,   // 70% sustain level
    release: 0.3    // Smooth release
};
```

### Audio Chain Structure
```javascript
// Always connect through the master chain
oscillator.connect(filter);
filter.connect(gainNode);
gainNode.connect(this.synthNodes[channel]);
// synthNodes[channel] connects to masterGain -> compressor -> reverb -> destination
```

## 🎮 Game Mechanics Guidelines

### Performance Considerations
- Use `requestAnimationFrame` for smooth animations
- Implement delta-time based movement for consistent speed
- Cache DOM elements in the `elements` object
- Use object pooling for particles when possible

### State Management
```javascript
// Always update gameState properties consistently
gameState.score += CONFIG.GAMEPLAY.POINTS_PER_FOOD;
gameState.level = Math.floor(gameState.snake.length / CONFIG.GAMEPLAY.LEVEL_THRESHOLD) + 1;

// Use the CONFIG object for all configurable values
const gridWidth = Math.floor(CONFIG.VISUAL.CANVAS_WIDTH / CONFIG.VISUAL.GRID_SIZE);
```

## 🎨 Visual Effects Guidelines

### Particle System
```javascript
// Follow the existing particle pattern
particleSystem.createExplosion(x, y, intensity);

// Intensity should scale with game state
const intensity = Math.min(gameState.score / 100, 3);
```

### Color Scheme
- Stick to the greenscale neon theme
- Use CSS custom properties for consistency
- Apply glow effects with `box-shadow` and `text-shadow`
- Maintain high contrast for accessibility

## 🧪 Testing Guidelines

### Browser Testing
Test your changes in:
- **Chrome/Chromium** (latest)
- **Firefox** (latest)
- **Safari** (if on macOS)
- **Edge** (latest)

### Functionality Testing
- **Audio**: Test with sound on/off, check Web Audio API compatibility
- **Input**: Test all control schemes (WASD, arrow keys, ESC, space)
- **Performance**: Ensure smooth 60fps gameplay
- **Responsive**: Test on different screen sizes
- **High Scores**: Test localStorage functionality

### Game Testing Checklist
- [ ] Snake moves correctly in all directions
- [ ] Cannot reverse into self
- [ ] Food spawns in valid locations
- [ ] Collision detection works properly
- [ ] Score updates correctly
- [ ] Speed increases with snake length
- [ ] Music adapts to game state
- [ ] Particle effects trigger appropriately
- [ ] Text exclamations appear at milestones
- [ ] High score system functions
- [ ] Menu navigation works
- [ ] Audio mute/unmute functions

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Browser and version**
2. **Steps to reproduce**
3. **Expected behavior**
4. **Actual behavior**
5. **Console errors** (if any)
6. **Screenshots/recordings** (if visual issue)

### Bug Report Template
```markdown
## Bug Description
Brief description of the issue

## Steps to Reproduce
1. Open snek.html in browser
2. Click "Start Game"
3. Move snake with WASD
4. Bug occurs when...

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- Browser: Chrome 91.0.4472.124
- OS: macOS Big Sur 11.4
- Console Errors: [paste any errors]
```

## 💡 Feature Requests

We welcome feature requests! When suggesting new features:

1. **Check existing issues** first
2. **Describe the feature** and its benefits
3. **Consider technical feasibility**
4. **Propose implementation approach** if possible

### Good Feature Ideas
- New visual effects or themes
- Additional audio tracks or sound effects
- Game modes (multiplayer, challenges, etc.)
- Performance optimizations
- Accessibility improvements
- Mobile/touch support

## 🔧 Development Workflow

### Branch Naming
- `feature/synthwave-improvements`
- `bugfix/collision-detection`
- `refactor/audio-system`
- `docs/api-documentation`

### Commit Messages
```
feat: add new particle explosion patterns

- Implement spiral and burst patterns
- Scale effects with game intensity
- Optimize particle lifecycle management

Closes #42
```

### Pull Request Guidelines

1. **Clear title** describing the change
2. **Detailed description** of what was changed and why
3. **Reference issues** if applicable
4. **Test coverage** description
5. **Screenshots** for visual changes

## 📚 Resources

### Web Audio API
- [MDN Web Audio API Guide](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
- [Web Audio API Specification](https://www.w3.org/TR/webaudio/)

### HTML5 Canvas
- [MDN Canvas Tutorial](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial)
- [Canvas Performance Tips](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Optimizing_canvas)

### Synthwave Music Theory
- [Basic Chord Progressions](https://www.musictheory.net/lessons)
- [Synthesizer Programming](https://www.sound.org/ebooks/)

## 🏆 Recognition

Contributors will be recognized in:
- GitHub contributors list
- Project README.md
- Release notes for significant contributions

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and community chat
- **Code Review**: Submit PRs for collaborative improvement

## 🎯 Roadmap Priorities

Current focus areas:
1. **Performance optimization**
2. **Mobile/touch support**  
3. **Accessibility improvements**
4. **Modular architecture completion**
5. **Additional game modes**

Thank you for contributing to SNEK! Together we're building the ultimate retro-futuristic arcade experience. 🚀🐍✨