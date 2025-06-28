// Snake entity
class Snake {
    constructor(renderer, startPosition = { x: CONSTANTS.START_X, y: CONSTANTS.START_Y }) {
        this.renderer = renderer;
        this.segments = [{ ...startPosition }];
        this.direction = { x: 0, y: 0 };
        this.nextDirection = { x: 0, y: 0 };
        this.color = '#00ff00';
        this.hasGrown = false;
    }
    
    // Set new direction (will be applied on next move)
    setDirection(x, y) {
        // Prevent reversing into self
        if (this.direction.x !== 0 && x === -this.direction.x) return;
        if (this.direction.y !== 0 && y === -this.direction.y) return;
        
        this.nextDirection = { x, y };
    }
    
    // Move the snake one step
    move() {
        // Apply queued direction change
        if (this.nextDirection.x !== 0 || this.nextDirection.y !== 0) {
            this.direction = { ...this.nextDirection };
        }
        
        // Don't move if no direction set
        if (this.direction.x === 0 && this.direction.y === 0) return;
        
        // Calculate new head position
        const head = this.getHead();
        const newHead = {
            x: head.x + this.direction.x,
            y: head.y + this.direction.y
        };
        
        // Add new head
        this.segments.unshift(newHead);
        
        // Remove tail if not growing
        if (!this.hasGrown) {
            this.segments.pop();
        } else {
            this.hasGrown = false;
        }
    }
    
    // Grow the snake (don't remove tail on next move)
    grow() {
        this.hasGrown = true;
    }
    
    // Check if snake collides with walls
    checkWallCollision() {
        const head = this.getHead();
        const gridDims = this.renderer.getGridDimensions();
        
        return (
            head.x < 0 || 
            head.x >= gridDims.width || 
            head.y < 0 || 
            head.y >= gridDims.height
        );
    }
    
    // Check if snake collides with itself
    checkSelfCollision() {
        const head = this.getHead();
        
        // Check against all segments except the head
        for (let i = 1; i < this.segments.length; i++) {
            if (Utils.positionsEqual(head, this.segments[i])) {
                return true;
            }
        }
        
        return false;
    }
    
    // Check if head is at given position
    isHeadAt(position) {
        return Utils.positionsEqual(this.getHead(), position);
    }
    
    // Get head position
    getHead() {
        return this.segments[0];
    }
    
    // Get all segments
    getSegments() {
        return [...this.segments];
    }
    
    // Get previous tail position (for selective rendering)
    getPreviousTail(previousSegments) {
        if (previousSegments && previousSegments.length > 0) {
            return previousSegments[previousSegments.length - 1];
        }
        return null;
    }
    
    // Render the snake
    render() {
        this.segments.forEach(segment => {
            this.renderer.drawRect(segment.x, segment.y, this.color);
        });
    }
    
    // Reset snake to starting position
    reset(startPosition = { x: CONSTANTS.START_X, y: CONSTANTS.START_Y }) {
        this.segments = [{ ...startPosition }];
        this.direction = { x: 0, y: 0 };
        this.nextDirection = { x: 0, y: 0 };
        this.hasGrown = false;
    }
}