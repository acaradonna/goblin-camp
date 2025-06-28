# 🎮 Web Games & Interactive Applications

This repository contains two innovative web applications that showcase modern web technologies and creative user experiences.

## 📁 Projects

### 🐍 [SNEK](./snek/) - Future Retro Arcade Experience
An epic arcade-style implementation of the classic Snake game with neon greenscale aesthetics, dynamic synthwave music, and particle explosions.

**Features:**
- Procedural synthwave audio system
- Neon greenscale visual theme with particle effects
- Progressive difficulty and speed scaling
- High score persistence
- Modular, testable architecture

### 🎥 [EmojiVision](./emojivision/) - Real-time Webcam to Emoji Art
A real-time webcam-to-emoji art converter that transforms your live video feed into ASCII art using emojis.

**Features:**
- Live webcam processing
- Smart brightness mapping
- Performance optimized for real-time display
- Snapshot capture functionality
- Responsive design

## 🚀 Quick Start

### Using Docker (Recommended)

```bash
# Run both applications
docker-compose up

# Run individual applications
docker-compose up snek          # SNEK Game on http://localhost:3000
docker-compose up emojivision   # EmojiVision on http://localhost:3001

# Run with reverse proxy (optional)
docker-compose --profile traefik up
```

With Traefik reverse proxy:
- **SNEK**: http://snek.localhost
- **EmojiVision**: http://emojivision.localhost  
- **Traefik Dashboard**: http://localhost:8080

### Manual Setup

#### SNEK Game
```bash
cd snek/
npm install
npm run dev    # Development server
npm run build  # Production build
```

#### EmojiVision
```bash
cd emojivision/
# Simply open index.html in a modern browser
# Or serve with any static file server
python -m http.server 8000
```

## 🏗️ Architecture

### Repository Structure
```
.
├── snek/                    # Snake game with modern build setup
│   ├── src/                # Modular source code (in development)
│   ├── dist/               # Built production files
│   ├── tests/              # Jest test suite
│   ├── docker/             # Docker configuration
│   └── package.json        # Node.js dependencies and scripts
├── emojivision/            # Webcam emoji converter
│   ├── index.html          # Complete single-file application
│   └── README.md           # Application documentation
├── docker-compose.yml      # Multi-service orchestration
├── Dockerfile.snek         # SNEK production build
├── Dockerfile.emojivision  # EmojiVision static serving
└── README.md              # This file
```

### Technology Stack

#### SNEK
- **Frontend**: Vanilla JavaScript (ES6+), HTML5 Canvas, CSS3
- **Audio**: Web Audio API for real-time synthwave generation
- **Build**: Webpack, Babel, PostCSS
- **Testing**: Jest with JSDOM
- **Linting**: ESLint + Prettier
- **Deployment**: Docker + Nginx

#### EmojiVision  
- **Frontend**: Vanilla JavaScript, HTML5 Canvas, CSS3
- **APIs**: MediaDevices (webcam), Canvas (image processing)
- **Deployment**: Docker + Nginx (static files)

## 🐳 Docker Configuration

### Services
- **snek**: Production-built Snake game (port 3000)
- **emojivision**: Static file server for emoji converter (port 3001)
- **traefik**: Optional reverse proxy with dashboard (port 8080)

### Build Process
Both applications use multi-stage Docker builds for optimized production images:

1. **SNEK**: Build stage (Node.js) → Production stage (Nginx)
2. **EmojiVision**: Direct Nginx serving of static files

### Environment Configuration
- Production-ready with health checks
- Automatic restarts
- Gzip compression
- Security headers
- Static asset caching

## 🛠️ Development

### Prerequisites
- **Docker & Docker Compose** (recommended)
- **Node.js 16+** (for SNEK development)
- **Modern web browser** with camera support

### Development Workflow

```bash
# Clone the repository
git clone <repository-url>
cd repos/

# Start development environment
docker-compose up

# For SNEK development with hot reload
cd snek/
npm install
npm run dev

# Run tests
npm test

# Build for production
npm run build
```

### Testing
```bash
# SNEK: Full test suite with Jest
cd snek/
npm test                # Run all tests
npm run test:watch      # Watch mode for development
npm run test:coverage   # Generate coverage report

# EmojiVision: Browser-based testing
# Open application in browser and test functionality
```

### Code Quality
```bash
# SNEK: Automated linting and formatting
cd snek/
npm run lint           # Check for issues
npm run lint:fix       # Auto-fix issues
npm run format         # Format with Prettier
```

## 🚀 Deployment

### Docker Production
```bash
# Build production images
docker build -f Dockerfile.snek -t snek-game:latest .
docker build -f Dockerfile.emojivision -t emojivision:latest .

# Deploy with compose
docker-compose -f docker-compose.yml up -d
```

### Manual Deployment
- **SNEK**: `npm run build` → serve `dist/` directory
- **EmojiVision**: Serve root directory with static file server

## 🔧 Configuration

### Environment Variables
- `NODE_ENV`: Development/production mode for SNEK
- Custom Docker environment variables supported

### Security
- HTTPS required for camera access (EmojiVision)
- Content Security Policy headers
- XSS protection enabled
- Frame options configured

## 📝 Browser Support

### SNEK
- Chrome 60+, Firefox 55+, Safari 11+, Edge 79+
- Requires Web Audio API support

### EmojiVision  
- Chrome 53+, Firefox 36+, Safari 11+, Edge 12+
- Requires `getUserMedia` API for camera access

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following the existing code style
4. Add tests for new functionality (SNEK)
5. Ensure all tests pass
6. Submit a pull request

### Development Guidelines
- Use modern JavaScript (ES6+)
- Follow existing code style and conventions
- Write tests for new features (SNEK)
- Update documentation as needed
- Test in multiple browsers

## 📄 License

This project is licensed under the MIT License - see individual project files for details.

## 🏆 Features & Highlights

### Innovation
- **Real-time audio synthesis** (SNEK)
- **Live video processing** (EmojiVision)
- **Progressive Web App features**
- **Modern containerized deployment**

### Performance
- Optimized Canvas rendering
- Efficient frame processing
- Minimal dependencies
- Production-ready builds

### User Experience
- Responsive design
- Accessibility considerations
- Cross-platform compatibility
- Intuitive controls

---

**Made with ❤️ and modern web technologies**