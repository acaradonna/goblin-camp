# 🐍 SNEK - Future Retro Arcade Experience

An epic arcade-style implementation of the classic Snake game with neon greenscale aesthetics, dynamic nintendostep music, particle explosions, and modular architecture designed for easy customization.

## ✨ Features

### 🎮 Gameplay
- **Dynamic Speed**: Game accelerates as snake grows longer
- **Level System**: Progressive difficulty with visual feedback
- **Responsive Controls**: WASD/Arrow keys with smooth movement
- **Smart Collision**: Precise wall and self-collision detection

### 🎨 Visual Experience
- **Neon Greenscale Theme**: Geometry Wars-inspired glow effects
- **Particle Explosions**: Green pixel art bursts when eating food
- **Text Exclamations**: "ALL RIIIIGHT!", "SNEK!!!!", "WOW!!!" animations
- **Pulsing Food**: Dynamic visual effects that scale with intensity
- **Glowing Snake**: Head-to-tail brightness gradient with neon glow

### 🎵 Procedural Synthwave Audio System
- **Menu Music "Neon Dreams"**: Atmospheric 70 BPM ambient synthwave with analog pads
- **Gameplay Music "Digital Highway"**: Dynamic 120-180 BPM synthwave that escalates with intensity
- **Real-time Synthesis**: No audio files needed - everything generated procedurally
- **Professional Audio Chain**: Master compression, convolution reverb, individual channel mixing
- **Adaptive Composition**: Music responds to score, level, and game state in real-time
- **Authentic 80s Sound**: ADSR envelopes, analog-style filters, classic waveforms

### 🏆 Progression
- **High Score System**: Persistent 3-character leaderboard
- **Achievement Triggers**: Score and length-based exclamations
- **Visual Feedback**: Particle intensity scales with performance
- **Speed Multiplier**: Real-time speed display in HUD

### 🔧 Architecture
- **Modular Design**: Easy to customize and extend
- **Configuration System**: Centralized settings for all game aspects
- **Clean Separation**: Audio, Visual, and Game logic isolated
- **Performance Optimized**: RequestAnimationFrame with delta timing

## 🎮 How to Play

### Basic Controls
- Use **WASD** or **Arrow Keys** to move the snake
- Press **ESC** to return to main menu anytime
- Press **Space** to restart after game over
- Click **🔊** button to toggle audio

### Gameplay Mechanics
- Eat the **glowing red food** to grow and increase your score
- Avoid hitting the **walls** or your **own body**
- **Speed increases** as your snake grows longer
- **Music intensifies** with your performance
- Achieve **high scores** for epic text exclamations
- Experience **particle explosions** when eating food

## 🚀 Quick Start

### Current Version (Single File)
```bash
# Clone the repository
git clone <repository-url>
cd sn8k

# Open SNEK in browser
open snek.html
```

### Features in Current Version
- ✅ **Procedural Synthwave Audio System**
- ✅ **Neon Greenscale Visual Theme**
- ✅ **Dynamic Particle Effects**
- ✅ **Arcade-Style Main Menu**
- ✅ **Text Exclamation System**
- ✅ **Progressive Speed/Difficulty**
- ✅ **High Score Persistence**

### Development Setup
```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run tests
npm test

# Lint and format code
npm run lint
npm run format
```

### 🐳 Docker Deployment

#### Quick Start with Docker
```bash
# Build and run SNEK game
docker build -f ../Dockerfile.snek -t snek-game ..
docker run -p 3000:80 snek-game

# Or use docker-compose for both applications
docker-compose up snek
```

#### Production Deployment
```bash
# Build production image
docker build -f ../Dockerfile.snek -t snek-game:latest ..

# Run with custom configuration
docker run -d \
  --name snek-production \
  -p 3000:80 \
  --restart unless-stopped \
  snek-game:latest
```

#### Development with Docker
```bash
# Run both applications
docker-compose up

# Run with Traefik reverse proxy
docker-compose --profile traefik up

# Access applications:
# - SNEK: http://localhost:3000 or http://snek.localhost (with Traefik)
# - EmojiVision: http://localhost:3001 or http://emojivision.localhost (with Traefik)
# - Traefik Dashboard: http://localhost:8080 (with Traefik profile)
```

## 🏗️ Architecture

This project is currently being refactored from a single HTML file to a modern, modular architecture. See [REFACTOR-PLAN.md](./REFACTOR-PLAN.md) for detailed information about the planned improvements.

### Current Structure
- `snake.html` - Monolithic game implementation

### Target Structure (In Progress)
```
src/
├── js/
│   ├── core/          # Game engine core
│   ├── entities/      # Game objects (Snake, Food)
│   ├── systems/       # Game systems (Audio, Input, Score)
│   └── utils/         # Utilities and constants
├── css/               # Modular stylesheets
└── index.html         # Clean HTML structure
```

## 🛠️ Technology Stack

- **Frontend**: Vanilla JavaScript (ES6+), HTML5 Canvas, CSS3
- **Audio**: Web Audio API for real-time synthwave generation
- **Typography**: Google Fonts (Orbitron) for retro-futuristic styling  
- **Storage**: localStorage for high score persistence
- **Architecture**: Modular class-based design with configuration system
- **Build**: Webpack (planned)
- **Deployment**: Docker + Nginx (planned)

## 🎯 Game Rules

1. **Objective**: Eat food to grow the snake and increase your score
2. **Movement**: Snake moves continuously in the current direction
3. **Scoring**: +10 points per food eaten
4. **Game Over**: Hitting walls or snake body ends the game
5. **High Scores**: Top 10 scores saved locally with player names

## 🔊 Synthwave Audio System

### 🌙 Menu Music: "Neon Dreams"
- **70 BPM** ambient synthwave atmosphere
- **Fm → Ab → Eb → Bb** chord progression with analog pads
- **Arpeggiated melodies** floating over dreamy soundscapes
- **Long reverb tails** and analog-style filtering

### ⚡ Gameplay Music: "Digital Highway"  
- **Dynamic BPM** (120-180) that scales with game difficulty
- **Driving bassline** with classic TB-303 style sawtooth waves
- **Epic lead synth** playing authentic 80s chord progressions
- **808 kick drums** and crispy hi-hats for that retro punch
- **Intensity scaling** - music evolves with your performance

### 🎛️ Technical Features
- **Real-time synthesis** - no audio files required
- **Professional audio chain** with compression and convolution reverb
- **ADSR envelopes** on every voice for authentic analog feel
- **Individual channel mixing** (bass, lead, pads, drums, arpeggios)
- **Adaptive composition** that responds to game state

## 🎨 Design Principles

- **Clean Code**: Single responsibility, dependency injection
- **Performance**: Optimized rendering and collision detection
- **Accessibility**: Keyboard navigation and audio controls
- **Responsive**: Works on different screen sizes
- **Modular**: Separate concerns for maintainability

## 🚧 Roadmap

### Phase 1: Foundation ✅
- [x] Basic game mechanics
- [x] Audio system
- [x] High score system
- [x] Performance optimizations

### Phase 2: Refactoring (In Progress)
- [ ] Modular architecture
- [ ] Build system setup
- [ ] Testing framework
- [ ] Docker containerization

### Phase 3: Enhancement (Planned)
- [ ] Multiple difficulty levels
- [ ] Power-ups and special items
- [ ] Multiplayer support
- [ ] Mobile touch controls
- [ ] Progressive Web App features

### Phase 4: Polish (Planned)
- [ ] Enhanced graphics and animations
- [ ] More audio tracks
- [ ] Achievement system
- [ ] Online leaderboards

## 🤝 Contributing

Contributions are welcome! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 Development

### Current Development
The game is currently in a single HTML file for simplicity. To make changes:

1. Edit `snake.html`
2. Test in browser
3. Ensure all features work correctly

### Future Development
Once refactored, development will use modern tooling:

```bash
# Development with hot reload
npm run dev

# Run tests
npm test

# Build for production
npm run build

# Docker development
docker-compose up
```

## 🔧 Configuration

Game settings can be modified in the Constants section:

```javascript
const GRID_SIZE = 20;        // Size of each grid cell
const gameSpeed = 150;       // Game update interval (ms)
```

## 📱 Browser Support

- Chrome 60+
- Firefox 55+
- Safari 11+
- Edge 79+

Requires Web Audio API support for sound features.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the classic Nokia Snake game
- Built with modern web technologies
- Audio synthesis using Web Audio API
- Canvas rendering for smooth gameplay

---

**Made with ❤️ and lots of ☕**