// Canvas rendering utilities
class CanvasRenderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.gridDimensions = Utils.getGridDimensions(
            canvas.width, 
            canvas.height, 
            CONSTANTS.GRID_SIZE
        );
    }
    
    // Clear the entire canvas
    clear() {
        this.ctx.fillStyle = '#000';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }
    
    // Clear a specific grid cell
    clearCell(x, y) {
        this.ctx.fillStyle = '#000';
        this.ctx.fillRect(
            x * CONSTANTS.GRID_SIZE, 
            y * CONSTANTS.GRID_SIZE, 
            CONSTANTS.GRID_SIZE, 
            CONSTANTS.GRID_SIZE
        );
    }
    
    // Draw a rectangle at grid position
    drawRect(x, y, color, padding = 2) {
        this.ctx.fillStyle = color;
        this.ctx.fillRect(
            x * CONSTANTS.GRID_SIZE, 
            y * CONSTANTS.GRID_SIZE, 
            CONSTANTS.GRID_SIZE - padding, 
            CONSTANTS.GRID_SIZE - padding
        );
    }
    
    // Get canvas dimensions in grid units
    getGridDimensions() {
        return this.gridDimensions;
    }
}