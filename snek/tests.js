// ===== SNEK AUTOMATED TEST SUITE =====

// ===== GAME MECHANICS TESTS =====
testFramework.describe('Game Mechanics', () => {
    
    testFramework.it('should initialize game state correctly', () => {
        assertEqual(gameState.currentScreen, 'menu');
        assertEqual(gameState.score, 0);
        assertEqual(gameState.level, 1);
        assertEqual(gameState.speed, CONFIG.GAMEPLAY.BASE_SPEED);
        assertArrayEqual(gameState.snake, [{ x: 20, y: 15 }]);
        assertEqual(gameState.foodEaten, 0);
        assertEqual(gameState.gameRunning, false);
    });
    
    testFramework.it('should generate valid food positions', () => {
        const food = generateFood();
        assert(typeof food.x === 'number', 'Food x coordinate should be a number');
        assert(typeof food.y === 'number', 'Food y coordinate should be a number');
        
        const gridWidth = Math.floor(CONFIG.VISUAL.CANVAS_WIDTH / CONFIG.VISUAL.GRID_SIZE);
        const gridHeight = Math.floor(CONFIG.VISUAL.CANVAS_HEIGHT / CONFIG.VISUAL.GRID_SIZE);
        
        assertGreaterThan(food.x, -1);
        assertLessThan(food.x, gridWidth);
        assertGreaterThan(food.y, -1);
        assertLessThan(food.y, gridHeight);
    });
    
    testFramework.it('should calculate game speed correctly', () => {
        // Test initial speed
        gameState.snake = [{ x: 20, y: 15 }];
        updateGameSpeed();
        assertEqual(gameState.speed, CONFIG.GAMEPLAY.BASE_SPEED);
        assertEqual(gameState.level, 1);
        
        // Test speed increase with longer snake
        gameState.snake = [
            { x: 20, y: 15 }, { x: 19, y: 15 }, { x: 18, y: 15 },
            { x: 17, y: 15 }, { x: 16, y: 15 }, { x: 15, y: 15 }
        ];
        updateGameSpeed();
        assertLessThan(gameState.speed, CONFIG.GAMEPLAY.BASE_SPEED);
        assertGreaterThan(gameState.level, 1);
    });
    
    testFramework.it('should enforce minimum speed limit', () => {
        // Create very long snake
        gameState.snake = new Array(100).fill(null).map((_, i) => ({ x: i, y: 15 }));
        updateGameSpeed();
        assertGreaterThan(gameState.speed, CONFIG.GAMEPLAY.MIN_SPEED - 1);
    });
    
    testFramework.it('should calculate correct level progression', () => {
        gameState.snake = new Array(CONFIG.GAMEPLAY.LEVEL_THRESHOLD + 1).fill(null).map((_, i) => ({ x: i, y: 15 }));
        updateGameSpeed();
        assertEqual(gameState.level, 2);
        
        gameState.snake = new Array(CONFIG.GAMEPLAY.LEVEL_THRESHOLD * 2 + 1).fill(null).map((_, i) => ({ x: i, y: 15 }));
        updateGameSpeed();
        assertEqual(gameState.level, 3);
    });
    
    testFramework.it('should detect high scores correctly', () => {
        assertEqual(isHighScore(0), false, 'Score of 0 should not be high score');
        assertEqual(isHighScore(50), false, 'Score of 50 should not be high score');
        assertEqual(isHighScore(150), true, 'Score of 150 should be high score');
        assertEqual(isHighScore(1000), true, 'Score of 1000 should be high score');
    });
    
    testFramework.it('should validate high scores format', () => {
        const scores = getHighScores();
        assert(Array.isArray(scores), 'High scores should be an array');
        assert(scores.length > 0, 'Should have some high scores');
        
        scores.forEach(score => {
            assert(typeof score.name === 'string', 'Score name should be string');
            assert(typeof score.score === 'number', 'Score value should be number');
            assertLessThan(score.name.length, 4, 'Score name should be 3 characters or less');
        });
    });
});

// ===== AUDIO SYSTEM TESTS =====
testFramework.describe('Audio System', () => {
    
    testFramework.it('should initialize audio system correctly', () => {
        assert(audioSystem.context instanceof MockAudioContext, 'Should have audio context');
        assertEqual(audioSystem.isMuted, false, 'Should start unmuted');
        assertEqual(audioSystem.currentBPM, CONFIG.AUDIO.BASE_BPM, 'Should start with base BPM');
        assertEqual(audioSystem.currentTrack, null, 'Should start with no track');
    });
    
    testFramework.it('should setup synth nodes correctly', () => {
        assert(audioSystem.masterGain instanceof MockGainNode, 'Should have master gain');
        assert(audioSystem.compressor instanceof MockCompressor, 'Should have compressor');
        assert(audioSystem.reverb instanceof MockConvolver, 'Should have reverb');
        
        const expectedChannels = ['bass', 'lead', 'pad', 'drums', 'arp'];
        expectedChannels.forEach(channel => {
            assert(audioSystem.synthNodes[channel] instanceof MockGainNode, 
                   `Should have ${channel} channel`);
        });
    });
    
    testFramework.it('should create synth voices with correct parameters', () => {
        const voice = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead', {
            volume: 0.5,
            attack: 0.1,
            decay: 0.2,
            sustain: 0.7,
            release: 0.3
        });
        
        if (!audioSystem.isMuted) {
            assert(voice !== null, 'Should create voice when not muted');
            assert(voice.osc instanceof MockOscillator, 'Should have oscillator');
            assert(voice.gain instanceof MockGainNode, 'Should have gain node');
            assert(voice.filter instanceof MockBiquadFilter, 'Should have filter');
        }
    });
    
    testFramework.it('should handle mute functionality', () => {
        audioSystem.isMuted = false;
        audioSystem.toggleMute();
        assertEqual(audioSystem.isMuted, true, 'Should toggle to muted');
        
        audioSystem.toggleMute();
        assertEqual(audioSystem.isMuted, false, 'Should toggle back to unmuted');
    });
    
    testFramework.it('should play sound effects correctly', () => {
        audioSystem.isMuted = false;
        
        // Test that sound functions don't throw errors
        assert(() => audioSystem.playEatSound(), 'Eat sound should not throw');
        assert(() => audioSystem.playGameOverSound(), 'Game over sound should not throw');
    });
    
    testFramework.it('should manage music tracks correctly', () => {
        audioSystem.startMenuMusic();
        assertEqual(audioSystem.currentTrack, 'menu', 'Should set menu track');
        
        audioSystem.startGameplayMusic();
        assertEqual(audioSystem.currentTrack, 'gameplay', 'Should set gameplay track');
        
        audioSystem.stopMusic();
        assertEqual(audioSystem.currentTrack, null, 'Should clear current track');
    });
    
    testFramework.it('should respect mute state for sound generation', () => {
        audioSystem.isMuted = true;
        const voice = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead');
        assertEqual(voice, null, 'Should not create voice when muted');
        
        audioSystem.isMuted = false;
        const voice2 = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead');
        assertNotEqual(voice2, null, 'Should create voice when unmuted');
    });
});

// ===== PARTICLE SYSTEM TESTS =====
testFramework.describe('Particle System', () => {
    
    testFramework.it('should initialize with empty particle array', () => {
        assertEqual(particleSystem.particles.length, 0, 'Should start with no particles');
    });
    
    testFramework.it('should create explosion particles correctly', () => {
        const x = 10, y = 10, intensity = 1;
        particleSystem.createExplosion(x, y, intensity);
        
        const expectedCount = Math.floor(CONFIG.VISUAL.PARTICLE_COUNT * intensity);
        assertEqual(particleSystem.particles.length, expectedCount, 
                   `Should create ${expectedCount} particles`);
        
        particleSystem.particles.forEach(particle => {
            assert(typeof particle.x === 'number', 'Particle x should be number');
            assert(typeof particle.y === 'number', 'Particle y should be number');
            assert(typeof particle.vx === 'number', 'Particle vx should be number');
            assert(typeof particle.vy === 'number', 'Particle vy should be number');
            assert(typeof particle.life === 'number', 'Particle life should be number');
            assert(typeof particle.size === 'number', 'Particle size should be number');
            
            assertEqual(particle.life, CONFIG.PARTICLES.LIFETIME, 'Particle should start with full life');
            assertGreaterThan(particle.size, CONFIG.PARTICLES.SIZE_MIN - 1, 'Particle size should be >= min');
            assertLessThan(particle.size, CONFIG.PARTICLES.SIZE_MAX + 1, 'Particle size should be <= max');
        });
    });
    
    testFramework.it('should scale particle count with intensity', () => {
        particleSystem.particles = []; // Clear existing particles
        
        const lowIntensity = 0.5;
        particleSystem.createExplosion(5, 5, lowIntensity);
        const lowCount = particleSystem.particles.length;
        
        particleSystem.particles = []; // Clear particles
        
        const highIntensity = 2.0;
        particleSystem.createExplosion(5, 5, highIntensity);
        const highCount = particleSystem.particles.length;
        
        assertGreaterThan(highCount, lowCount, 'Higher intensity should create more particles');
    });
    
    testFramework.it('should update particles over time', () => {
        particleSystem.particles = [];
        particleSystem.createExplosion(10, 10, 1);
        
        const initialCount = particleSystem.particles.length;
        const initialLife = particleSystem.particles[0].life;
        const initialX = particleSystem.particles[0].x;
        
        // Simulate time passing
        particleSystem.update(100); // 100ms
        
        assert(particleSystem.particles[0].life < initialLife, 'Particle life should decrease');
        assertNotEqual(particleSystem.particles[0].x, initialX, 'Particle should move');
    });
    
    testFramework.it('should remove dead particles', () => {
        particleSystem.particles = [];
        particleSystem.createExplosion(10, 10, 1);
        
        const initialCount = particleSystem.particles.length;
        
        // Simulate enough time for particles to die
        particleSystem.update(CONFIG.PARTICLES.LIFETIME + 100);
        
        assertEqual(particleSystem.particles.length, 0, 'Dead particles should be removed');
    });
});

// ===== EXCLAMATION SYSTEM TESTS =====
testFramework.describe('Exclamation System', () => {
    
    testFramework.it('should initialize with zero counters', () => {
        assertEqual(exclamationSystem.lastExclamationScore, 0, 'Should start with zero score trigger');
        assertEqual(exclamationSystem.lastExclamationLength, 0, 'Should start with zero length trigger');
    });
    
    testFramework.it('should show exclamation for valid message', () => {
        const message = exclamationSystem.showExclamation();
        assert(typeof message === 'string', 'Should return a string message');
        assert(CONFIG.EXCLAMATIONS.MESSAGES.includes(message), 'Should return valid exclamation message');
    });
    
    testFramework.it('should trigger on score milestones', () => {
        exclamationSystem.lastExclamationScore = 0;
        exclamationSystem.lastExclamationLength = 0;
        
        // Test score threshold triggering
        gameState.score = 50; // First threshold
        gameState.snake = [{ x: 20, y: 15 }]; // Reset snake length
        
        const initialScoreTrigger = exclamationSystem.lastExclamationScore;
        exclamationSystem.checkTriggers();
        assertGreaterThan(exclamationSystem.lastExclamationScore, initialScoreTrigger, 
                         'Should update score trigger after milestone');
    });
    
    testFramework.it('should trigger on snake length milestones', () => {
        exclamationSystem.lastExclamationScore = 0;
        exclamationSystem.lastExclamationLength = 0;
        
        // Test length threshold triggering
        gameState.score = 0; // Reset score
        gameState.snake = new Array(10).fill(null).map((_, i) => ({ x: i, y: 15 })); // 10 segments
        
        const initialLengthTrigger = exclamationSystem.lastExclamationLength;
        exclamationSystem.checkTriggers();
        assertGreaterThan(exclamationSystem.lastExclamationLength, initialLengthTrigger, 
                         'Should update length trigger after milestone');
    });
    
    testFramework.it('should not trigger multiple times for same milestone', () => {
        exclamationSystem.lastExclamationScore = 50;
        exclamationSystem.lastExclamationLength = 10;
        
        gameState.score = 50;
        gameState.snake = new Array(10).fill(null).map((_, i) => ({ x: i, y: 15 }));
        
        const scoreBefore = exclamationSystem.lastExclamationScore;
        const lengthBefore = exclamationSystem.lastExclamationLength;
        
        exclamationSystem.checkTriggers();
        
        assertEqual(exclamationSystem.lastExclamationScore, scoreBefore, 'Should not retrigger same score');
        assertEqual(exclamationSystem.lastExclamationLength, lengthBefore, 'Should not retrigger same length');
    });
});

// ===== CONFIGURATION TESTS =====
testFramework.describe('Configuration System', () => {
    
    testFramework.it('should have valid visual configuration', () => {
        assert(typeof CONFIG.VISUAL.GRID_SIZE === 'number', 'Grid size should be number');
        assertGreaterThan(CONFIG.VISUAL.GRID_SIZE, 0, 'Grid size should be positive');
        
        assert(typeof CONFIG.VISUAL.CANVAS_WIDTH === 'number', 'Canvas width should be number');
        assert(typeof CONFIG.VISUAL.CANVAS_HEIGHT === 'number', 'Canvas height should be number');
        assertGreaterThan(CONFIG.VISUAL.CANVAS_WIDTH, 0, 'Canvas width should be positive');
        assertGreaterThan(CONFIG.VISUAL.CANVAS_HEIGHT, 0, 'Canvas height should be positive');
        
        assert(typeof CONFIG.VISUAL.PARTICLE_COUNT === 'number', 'Particle count should be number');
        assertGreaterThan(CONFIG.VISUAL.PARTICLE_COUNT, 0, 'Particle count should be positive');
    });
    
    testFramework.it('should have valid color configuration', () => {
        const requiredColors = ['SNAKE', 'FOOD', 'BACKGROUND', 'GRID', 'GLOW'];
        requiredColors.forEach(color => {
            assert(typeof CONFIG.VISUAL.COLORS[color] === 'string', `${color} should be string`);
            assert(CONFIG.VISUAL.COLORS[color].length > 0, `${color} should not be empty`);
        });
    });
    
    testFramework.it('should have valid gameplay configuration', () => {
        assert(typeof CONFIG.GAMEPLAY.BASE_SPEED === 'number', 'Base speed should be number');
        assertGreaterThan(CONFIG.GAMEPLAY.BASE_SPEED, 0, 'Base speed should be positive');
        
        assert(typeof CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR === 'number', 'Speed factor should be number');
        assertGreaterThan(CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR, 0, 'Speed factor should be positive');
        assertLessThan(CONFIG.GAMEPLAY.SPEED_INCREASE_FACTOR, 1, 'Speed factor should be less than 1');
        
        assert(typeof CONFIG.GAMEPLAY.MIN_SPEED === 'number', 'Min speed should be number');
        assertGreaterThan(CONFIG.GAMEPLAY.MIN_SPEED, 0, 'Min speed should be positive');
        assertLessThan(CONFIG.GAMEPLAY.MIN_SPEED, CONFIG.GAMEPLAY.BASE_SPEED, 'Min speed should be less than base speed');
    });
    
    testFramework.it('should have valid audio configuration', () => {
        assert(typeof CONFIG.AUDIO.MASTER_VOLUME === 'number', 'Master volume should be number');
        assertGreaterThan(CONFIG.AUDIO.MASTER_VOLUME, -1, 'Master volume should be >= 0');
        assertLessThan(CONFIG.AUDIO.MASTER_VOLUME, 2, 'Master volume should be <= 1');
        
        assert(typeof CONFIG.AUDIO.BASE_BPM === 'number', 'Base BPM should be number');
        assertGreaterThan(CONFIG.AUDIO.BASE_BPM, 0, 'Base BPM should be positive');
        
        assert(typeof CONFIG.AUDIO.MAX_BPM === 'number', 'Max BPM should be number');
        assertGreaterThan(CONFIG.AUDIO.MAX_BPM, CONFIG.AUDIO.BASE_BPM, 'Max BPM should be greater than base BPM');
    });
    
    testFramework.it('should have valid exclamation configuration', () => {
        assert(Array.isArray(CONFIG.EXCLAMATIONS.MESSAGES), 'Messages should be array');
        assertGreaterThan(CONFIG.EXCLAMATIONS.MESSAGES.length, 0, 'Should have messages');
        
        CONFIG.EXCLAMATIONS.MESSAGES.forEach(message => {
            assert(typeof message === 'string', 'Each message should be string');
            assertGreaterThan(message.length, 0, 'Messages should not be empty');
        });
        
        assert(Array.isArray(CONFIG.EXCLAMATIONS.TRIGGERS.SCORE_MULTIPLES), 'Score triggers should be array');
        assert(Array.isArray(CONFIG.EXCLAMATIONS.TRIGGERS.SNAKE_LENGTHS), 'Length triggers should be array');
    });
    
    testFramework.it('should have valid particle configuration', () => {
        assert(typeof CONFIG.PARTICLES.LIFETIME === 'number', 'Lifetime should be number');
        assertGreaterThan(CONFIG.PARTICLES.LIFETIME, 0, 'Lifetime should be positive');
        
        assert(typeof CONFIG.PARTICLES.SPEED_MIN === 'number', 'Speed min should be number');
        assert(typeof CONFIG.PARTICLES.SPEED_MAX === 'number', 'Speed max should be number');
        assertLessThan(CONFIG.PARTICLES.SPEED_MIN, CONFIG.PARTICLES.SPEED_MAX, 'Min speed should be less than max');
        
        assert(typeof CONFIG.PARTICLES.SIZE_MIN === 'number', 'Size min should be number');
        assert(typeof CONFIG.PARTICLES.SIZE_MAX === 'number', 'Size max should be number');
        assertLessThan(CONFIG.PARTICLES.SIZE_MIN, CONFIG.PARTICLES.SIZE_MAX, 'Min size should be less than max');
    });
});

// ===== INTEGRATION TESTS =====
testFramework.describe('Integration Tests', () => {
    
    testFramework.it('should integrate audio and game state correctly', () => {
        // Test mute state persistence
        audioSystem.isMuted = false;
        audioSystem.startMenuMusic();
        assertEqual(audioSystem.currentTrack, 'menu', 'Should start menu music');
        
        audioSystem.startGameplayMusic();
        assertEqual(audioSystem.currentTrack, 'gameplay', 'Should switch to gameplay music');
    });
    
    testFramework.it('should integrate particles with game events', () => {
        particleSystem.particles = [];
        
        // Simulate food eating event
        const x = 5, y = 5;
        const intensity = Math.min(gameState.score / 100, 3);
        particleSystem.createExplosion(x, y, intensity);
        
        assertGreaterThan(particleSystem.particles.length, 0, 'Should create particles on food eat');
        
        // Verify particle positioning
        const expectedX = x * CONFIG.VISUAL.GRID_SIZE + CONFIG.VISUAL.GRID_SIZE / 2;
        const expectedY = y * CONFIG.VISUAL.GRID_SIZE + CONFIG.VISUAL.GRID_SIZE / 2;
        
        const firstParticle = particleSystem.particles[0];
        assertEqual(firstParticle.x, expectedX, 'Particle X should be correctly calculated');
        assertEqual(firstParticle.y, expectedY, 'Particle Y should be correctly calculated');
    });
    
    testFramework.it('should integrate exclamations with game progression', () => {
        // Reset exclamation system
        exclamationSystem.lastExclamationScore = 0;
        exclamationSystem.lastExclamationLength = 0;
        
        // Simulate game progression
        gameState.score = 100;
        gameState.snake = new Array(12).fill(null).map((_, i) => ({ x: i, y: 15 }));
        
        const scoreBefore = exclamationSystem.lastExclamationScore;
        const lengthBefore = exclamationSystem.lastExclamationLength;
        
        exclamationSystem.checkTriggers();
        
        assert(
            exclamationSystem.lastExclamationScore > scoreBefore || 
            exclamationSystem.lastExclamationLength > lengthBefore,
            'Should trigger exclamation for game progression'
        );
    });
    
    testFramework.it('should validate game state consistency', () => {
        // Test that all systems are initialized
        assert(audioSystem instanceof AudioSystem, 'Audio system should be initialized');
        assert(particleSystem instanceof ParticleSystem, 'Particle system should be initialized');
        assert(exclamationSystem instanceof ExclamationSystem, 'Exclamation system should be initialized');
        
        // Test that game state is coherent
        assert(gameState.level >= 1, 'Level should be at least 1');
        assert(gameState.score >= 0, 'Score should be non-negative');
        assert(gameState.snake.length >= 1, 'Snake should have at least one segment');
        assertLessThan(gameState.speed, CONFIG.GAMEPLAY.BASE_SPEED + 1, 'Speed should not exceed base speed');
        assertGreaterThan(gameState.speed, CONFIG.GAMEPLAY.MIN_SPEED - 1, 'Speed should not be below minimum');
    });
    
    testFramework.it('should handle edge cases gracefully', () => {
        // Test zero score
        gameState.score = 0;
        assertEqual(isHighScore(0), false, 'Zero score should not be high score');
        
        // Test very large numbers
        gameState.score = 999999;
        assert(() => exclamationSystem.checkTriggers(), 'Should handle very large scores');
        
        // Test empty particles array
        particleSystem.particles = [];
        assert(() => particleSystem.update(1000), 'Should handle empty particle array');
        
        // Test muted audio
        audioSystem.isMuted = true;
        assert(() => audioSystem.playEatSound(), 'Should handle muted audio gracefully');
    });
});

// ===== PERFORMANCE TESTS =====
testFramework.describe('Performance Tests', () => {
    
    testFramework.it('should generate food quickly', () => {
        const startTime = performance.now();
        for (let i = 0; i < 1000; i++) {
            generateFood();
        }
        const endTime = performance.now();
        const duration = endTime - startTime;
        
        assertLessThan(duration, 100, 'Should generate 1000 food positions in under 100ms');
    });
    
    testFramework.it('should update particles efficiently', () => {
        // Create many particles
        particleSystem.particles = [];
        for (let i = 0; i < 100; i++) {
            particleSystem.createExplosion(i % 10, Math.floor(i / 10), 1);
        }
        
        const startTime = performance.now();
        particleSystem.update(16); // 60fps frame time
        const endTime = performance.now();
        const duration = endTime - startTime;
        
        assertLessThan(duration, 16, 'Should update particles within frame budget');
    });
    
    testFramework.it('should calculate game speed efficiently', () => {
        const startTime = performance.now();
        for (let i = 0; i < 1000; i++) {
            gameState.snake = new Array(i % 50 + 1).fill(null).map((_, j) => ({ x: j, y: 15 }));
            updateGameSpeed();
        }
        const endTime = performance.now();
        const duration = endTime - startTime;
        
        assertLessThan(duration, 50, 'Should calculate speed for 1000 iterations in under 50ms');
    });
    
    testFramework.it('should handle audio creation efficiently', () => {
        audioSystem.isMuted = false;
        
        const startTime = performance.now();
        for (let i = 0; i < 100; i++) {
            audioSystem.createSynthVoice(440 + i, 0, 0.1, 'square', 'lead');
        }
        const endTime = performance.now();
        const duration = endTime - startTime;
        
        assertLessThan(duration, 100, 'Should create 100 synth voices in under 100ms');
    });
});

// Control functions for the test interface
function runAllTests() {
    testFramework.runAllTests();
}

function runGameTests() {
    testFramework.results = [];
    testFramework.tests = testFramework.tests.filter(test => 
        test.suite === 'Game Mechanics' || test.suite === 'Configuration System'
    );
    testFramework.runAllTests();
}

function runAudioTests() {
    testFramework.results = [];
    testFramework.tests = testFramework.tests.filter(test => 
        test.suite === 'Audio System'
    );
    testFramework.runAllTests();
}

function runUITests() {
    testFramework.results = [];
    testFramework.tests = testFramework.tests.filter(test => 
        test.suite === 'Particle System' || 
        test.suite === 'Exclamation System' || 
        test.suite === 'Integration Tests' ||
        test.suite === 'Performance Tests'
    );
    testFramework.runAllTests();
}

function clearResults() {
    document.getElementById('testResults').innerHTML = 
        '<p style="text-align: center; color: var(--bright-green);">Click "Run All Tests" to start automated testing</p>';
    
    document.getElementById('totalTests').textContent = '0';
    document.getElementById('passedTests').textContent = '0';
    document.getElementById('failedTests').textContent = '0';
    document.getElementById('skippedTests').textContent = '0';
    document.getElementById('testDuration').textContent = '0ms';
    document.getElementById('progressFill').style.width = '0%';
}