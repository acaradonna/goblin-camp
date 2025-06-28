# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the EmojiVision project.

## Project Overview
EmojiVision is a real-time webcam-to-emoji art converter built with vanilla JavaScript. It's a single-file application that processes live video feeds and converts them to ASCII art using emojis.

## Technology Stack
- **Vanilla JavaScript** (ES6+)
- **HTML5 Canvas API** for video processing
- **MediaDevices API** for webcam access
- **CSS3** for styling and responsive layout

## Development Guidelines
- Keep the single-file architecture for simplicity
- Use modern JavaScript features (const/let, arrow functions, async/await)
- Ensure camera permissions are handled gracefully
- Optimize for real-time performance (60fps target)
- Test across different browsers and devices

## Key Components
- Video capture and stream management
- Frame processing and brightness calculation
- Emoji mapping and rendering
- Snapshot functionality
- Quality/resolution controls

## Performance Considerations
- Use requestAnimationFrame for smooth updates
- Process at lower resolution than display
- Implement frame skipping to prevent overlap
- Optimize emoji rendering with monospace fonts

## Browser Compatibility
- Modern browsers with getUserMedia support
- HTTPS required for camera access
- Mobile browsers with camera support

## Deployment
- Static file serving (any HTTP server)
- Docker containerization available
- No build process required
