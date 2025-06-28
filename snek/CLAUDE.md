# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the SNEK project.

## Project Overview
SNEK is a modern, feature-rich implementation of the classic Snake game with synthwave aesthetics, procedural audio generation, and advanced visual effects. The project is transitioning from a monolithic single-file architecture to a modular, testable codebase.

## Technology Stack
- **Vanilla JavaScript** (ES6+) with modern features
- **HTML5 Canvas API** for game rendering
- **Web Audio API** for real-time synthwave music generation
- **CSS3** with neon greenscale theming
- **Webpack** for build tooling and module bundling
- **Jest** for unit testing with JSDOM environment
- **ESLint + Prettier** for code quality
- **Docker** for containerized deployment

## Architecture Status
Currently in **Phase 2: Refactoring** (see REFACTOR-PLAN.md)
- **Current**: Monolithic `snek.html` file (functional)
- **Target**: Modular `src/` directory structure (in progress)
- **Build System**: Webpack configuration ready
- **Testing**: Jest framework configured

## Development Guidelines

### Code Style
- Use ES6+ features (classes, arrow functions, async/await)
- Follow the existing configuration system pattern
- Implement single responsibility principle
- Use descriptive function and variable names
- Keep functions pure when possible

### Audio System
- All audio generated procedurally (no audio files)
- Use ADSR envelopes for authentic analog feel
- Connect through master audio chain (compression + reverb)
- Frequencies based on standard musical notes
- Adaptive composition responding to game state

### Visual System
- Maintain neon greenscale theme
- Use CSS custom properties for theming
- Implement smooth animations with requestAnimationFrame
- Scale particle effects with game intensity
- Ensure responsive design across screen sizes

### Game Logic
- Delta-time based movement for consistent speed
- Efficient collision detection
- Centralized state management
- Event-driven architecture
- Performance optimization (avoid memory leaks)

## Key Components

### Core Systems
- **Canvas**: Rendering engine and game loop
- **AudioSystem**: Procedural synthwave generation
- **ParticleSystem**: Visual effects and explosions
- **ExclamationSystem**: Achievement text animations

### Game Entities
- **Snake**: Player character with head-to-tail gradient
- **Food**: Animated food items with pulsing effects

### Utilities
- **Constants**: Centralized configuration
- **GameState**: State management
- **Utils**: Helper functions

## Testing Strategy
- **Unit Tests**: Core game logic and systems
- **Integration Tests**: Component interactions
- **Performance Tests**: Frame rate and memory usage
- **Browser Tests**: Cross-platform compatibility

### Test Commands
```bash
npm test              # Run all tests
npm run test:watch    # Watch mode for development
npm run test:coverage # Generate coverage report
```

## Build Process
- **Development**: `npm run dev` (webpack-dev-server with hot reload)
- **Production**: `npm run build` (optimized bundle in dist/)
- **Docker**: Multi-stage build (Node.js build → Nginx serve)

### Build Commands
```bash
npm install           # Install dependencies
npm run dev          # Start development server
npm run build        # Build for production
npm run lint         # Check code quality
npm run format       # Format with Prettier
```

## Configuration System
All game settings are centralized in the CONFIG object:

```javascript
const CONFIG = {
    VISUAL: {
        GRID_SIZE: 20,
        CANVAS_WIDTH: 800,
        CANVAS_HEIGHT: 600,
        COLORS: { /* ... */ }
    },
    GAMEPLAY: {
        INITIAL_SPEED: 150,
        SPEED_INCREASE: 5,
        POINTS_PER_FOOD: 10
    },
    AUDIO: {
        MASTER_VOLUME: 0.7,
        MENU_BPM: 70,
        GAME_BPM_MIN: 120
    }
};
```

## Performance Considerations
- Use object pooling for particles
- Implement frame skipping for consistent performance
- Cache DOM elements and calculations
- Optimize canvas operations
- Monitor memory usage in long sessions

## Browser Compatibility
- **Chrome 60+** (primary target)
- **Firefox 55+** (good support)
- **Safari 11+** (WebKit quirks to consider)
- **Edge 79+** (Chromium-based)
- Requires Web Audio API support for audio features

## Deployment
- **Development**: webpack-dev-server on localhost:8080
- **Production**: Static files served by Nginx
- **Docker**: Multi-service setup with docker-compose
- **Health Checks**: Nginx endpoint monitoring

## File Structure (Target)
```
src/
├── js/
│   ├── core/          # Game engine (Canvas, GameLoop)
│   ├── entities/      # Game objects (Snake, Food)
│   ├── systems/       # Game systems (Audio, Particles, etc.)
│   └── utils/         # Utilities (Constants, Config, Utils)
├── css/               # Modular stylesheets
└── index.html         # Clean HTML entry point
```

## Common Development Tasks

### Adding New Features
1. Update CONFIG object with new settings
2. Create/update relevant system or entity
3. Add unit tests for new functionality
4. Update documentation
5. Test across browsers

### Audio Development
1. Use standard musical frequencies
2. Implement proper ADSR envelopes
3. Connect through master audio chain
4. Test with audio on/off states
5. Ensure Web Audio API compatibility

### Visual Effects
1. Maintain greenscale neon theme
2. Use CSS custom properties
3. Implement smooth animations
4. Scale effects with game state
5. Test on different screen sizes

## Debugging Tips
- Use browser DevTools Performance tab for frame analysis
- Monitor Web Audio API context state
- Check Canvas rendering performance
- Validate game state consistency
- Test with audio disabled for debugging

## Known Issues & Considerations
- Web Audio API requires user interaction to start
- Safari has specific Canvas performance characteristics
- Mobile browsers may have different input handling
- High DPI displays need special Canvas scaling
- Long gaming sessions should be memory-tested

## Contributing Guidelines
1. Follow existing code style and patterns
2. Add tests for new functionality
3. Update documentation for significant changes
4. Test in multiple browsers
5. Ensure performance doesn't degrade
6. Maintain the synthwave aesthetic theme