// Food entity
class Food {
    constructor(renderer) {
        this.renderer = renderer;
        this.position = this.generatePosition();
        this.color = '#ff0000';
    }
    
    // Generate new food position
    generatePosition() {
        const gridDims = this.renderer.getGridDimensions();
        return Utils.randomPosition(gridDims.width, gridDims.height);
    }
    
    // Regenerate food at new position, avoiding snake segments
    regenerate(snakeSegments = []) {
        let attempts = 0;
        const maxAttempts = 100;
        
        do {
            this.position = this.generatePosition();
            attempts++;
        } while (
            this.isOnSnake(snakeSegments) && 
            attempts < maxAttempts
        );
    }
    
    // Check if food is on any snake segment
    isOnSnake(snakeSegments) {
        return snakeSegments.some(segment => 
            Utils.positionsEqual(segment, this.position)
        );
    }
    
    // Check if position matches food position
    isAt(position) {
        return Utils.positionsEqual(position, this.position);
    }
    
    // Render the food
    render() {
        this.renderer.drawRect(this.position.x, this.position.y, this.color);
    }
    
    // Get current position
    getPosition() {
        return { ...this.position };
    }
}