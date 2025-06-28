// Utility functions
const Utils = {
    // Generate random position within grid bounds
    randomPosition(maxX, maxY) {
        return {
            x: Math.floor(Math.random() * maxX),
            y: Math.floor(Math.random() * maxY)
        };
    },
    
    // Check if two positions are equal
    positionsEqual(pos1, pos2) {
        return pos1.x === pos2.x && pos1.y === pos2.y;
    },
    
    // Clamp value between min and max
    clamp(value, min, max) {
        return Math.min(Math.max(value, min), max);
    },
    
    // Deep clone an object
    deepClone(obj) {
        return JSON.parse(JSON.stringify(obj));
    },
    
    // Get grid dimensions based on canvas size
    getGridDimensions(canvasWidth, canvasHeight, gridSize) {
        return {
            width: Math.floor(canvasWidth / gridSize),
            height: Math.floor(canvasHeight / gridSize)
        };
    },
    
    // Format name for high score (3 chars, uppercase)
    formatName(name) {
        return name.toUpperCase().padEnd(3, ' ').substring(0, 3);
    }
};