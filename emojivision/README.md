# 🎥 EmojiVision

A real-time webcam-to-emoji art converter that transforms your live video feed into ASCII art using emojis. Watch yourself become a living piece of emoji art!

## Features

- **Live Webcam Processing**: Real-time conversion of webcam feed to emoji art
- **Smart Brightness Mapping**: Uses luminance formula to map pixels to appropriate emojis
- **Performance Optimized**: Efficient frame processing for smooth real-time display
- **Snapshot Capture**: Save current emoji art frame as downloadable text file
- **Responsive Design**: Automatically adjusts to different screen sizes
- **Quality Settings**: Choose between Low, Medium, and High quality modes
- **Cross-Platform**: Works on desktop and mobile browsers

## How to Run

### Simple Setup
1. Clone or download this repository
2. Open `index.html` in any modern web browser
3. Click "Start Camera" and allow camera permissions
4. Enjoy your emoji transformation!

### Local Server (Optional)
For better performance and development, you can run a local server:

```bash
# Using Python 3
python -m http.server 8000

# Using Node.js (if you have http-server installed)
npx http-server

# Using PHP
php -S localhost:8000
```

Then open `http://localhost:8000` in your browser.

### 🐳 Docker Deployment

#### Quick Start with Docker
```bash
# Build and run EmojiVision
docker build -f ../Dockerfile.emojivision -t emojivision ..
docker run -p 3001:80 emojivision

# Or use docker-compose for both applications
docker-compose up emojivision
```

#### Production Deployment
```bash
# Build production image
docker build -f ../Dockerfile.emojivision -t emojivision:latest ..

# Run with custom configuration
docker run -d \
  --name emojivision-production \
  -p 3001:80 \
  --restart unless-stopped \
  emojivision:latest
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

## How It Works

### 1. Video Capture
- Uses `navigator.mediaDevices.getUserMedia()` to access the webcam
- Requests video stream with optimal resolution (640x480)
- Handles permission requests and error states gracefully

### 2. Frame Processing
```javascript
// Each video frame is processed through these steps:
1. Draw video frame to hidden canvas at reduced resolution
2. Extract pixel data using getImageData()
3. Calculate brightness for each pixel using luminance formula:
   brightness = (r * 0.299 + g * 0.587 + b * 0.114)
4. Map brightness value to emoji from darkest to lightest
5. Render emoji grid to display element
```

### 3. Emoji Mapping
The app uses 17 varied emojis arranged from darkest to lightest:
- 🕳️ (darkest - hole)
- 🌑 (new moon)
- 🌚 (dark moon face)
- 🎱 (8-ball)
- 🕷️ (spider)
- 🐜 (ant)
- 🦇 (bat)
- 🐧 (penguin)
- 🌪️ (tornado)
- 🌊 (ocean wave)
- 💙 (blue heart)
- 🌀 (cyclone)
- ❄️ (snowflake)
- ☁️ (cloud)
- 🤍 (white heart)
- 💫 (star)
- ✨ (lightest - sparkles)

### 4. Performance Optimizations
- **Frame Skipping**: Prevents processing overlap using `isProcessing` flag
- **Dynamic Resolution**: Quality settings adjust processing resolution
- **Efficient Rendering**: Uses `requestAnimationFrame` for smooth updates
- **Responsive Font Sizing**: Automatically adjusts emoji size to fit screen
- **Canvas Optimization**: Processes at lower resolution than display

## Browser Requirements

- **Camera Access**: Modern browsers with `getUserMedia` support
- **Canvas API**: For frame processing
- **ES6 Features**: Classes, arrow functions, async/await
- **File Download**: For snapshot functionality

### Supported Browsers
- Chrome 53+
- Firefox 36+
- Safari 11+
- Edge 12+
- Mobile browsers with camera support

## Controls

- **Start Camera**: Request webcam access and begin processing
- **Stop Camera**: Stop video feed and processing
- **📸 Snapshot**: Download current emoji art as text file
- **Quality Selector**: Choose processing resolution:
  - **Low**: 60x40 pixels (best performance)
  - **Medium**: 80x50 pixels (balanced)
  - **High**: 120x70 pixels (best quality)

## Technical Details

### Architecture
- **Single File**: Entire app contained in `index.html`
- **Vanilla JavaScript**: No external dependencies
- **CSS Grid/Flexbox**: Responsive layout
- **Canvas API**: Frame processing
- **Web APIs**: Camera access, file downloads

### Performance Considerations
- Processing resolution is independent of display resolution
- Frame processing is limited by `requestAnimationFrame` (~60fps max)
- Emoji rendering uses monospace font for consistent spacing
- Memory efficient - reuses canvas and doesn't store frame history

### File Structure
```
emojivision/
├── index.html          # Complete application
├── README.md           # This file
└── CLAUDE.md          # Development guidance
```

## Privacy

- **No Data Collection**: All processing happens locally in your browser
- **No Network Requests**: App works completely offline after loading
- **Camera Access**: Only used for real-time processing, no recording or storage
- **Snapshots**: Only saved when explicitly requested by user

## Troubleshooting

### Camera Not Working
- Ensure HTTPS or localhost (required for camera access)
- Check browser permissions for camera access
- Try refreshing the page
- Verify camera isn't being used by another application

### Performance Issues
- Switch to "Low" quality mode
- Close other browser tabs using camera
- Try a different browser
- Ensure good lighting for better contrast

### Display Problems
- Zoom out if emoji art appears cut off
- Try different quality settings
- Check if browser supports emoji rendering
- Refresh page if layout appears broken

## Future Enhancements

Potential improvements for future versions:
- Color emoji mapping (not just brightness)
- Custom emoji sets
- Video recording of emoji art
- Real-time effects and filters
- Face detection integration
- Multiple camera support

## License

This project is open source and available under the MIT License.