# SNEK Testing Guidelines

## 🧪 Testing Strategy

This document outlines our comprehensive testing approach for SNEK, ensuring code quality and preventing regressions.

## 📋 Testing Standards

### ✅ **MANDATORY: Unit Tests for Every Change**

**Every code change must include corresponding unit tests.**

1. **New Features**: Must include comprehensive test coverage (>90%)
2. **Bug Fixes**: Must include regression tests that would have caught the bug
3. **Refactoring**: Must maintain existing test coverage and add tests for new edge cases
4. **Configuration Changes**: Must include validation tests for new config options

### 🔄 **Test-Driven Development Workflow**

1. **Before Making Changes**:
   ```bash
   npm test  # Ensure all existing tests pass
   ```

2. **During Development**:
   ```bash
   npm run test:watch  # Run tests continuously during development
   ```

3. **Before Committing**:
   ```bash
   npm test              # Full test suite
   npm run lint          # Code quality check
   npm run test:coverage # Ensure coverage thresholds
   ```

## 📁 Test Organization

### **Test File Structure**
```
tests/
├── setup.js              # Jest configuration and mocks
├── config.test.js         # Configuration validation tests
├── gameState.test.js      # Game state management tests
├── AudioSystem.test.js    # Audio engine tests
├── ParticleSystem.test.js # Particle effects tests
├── ExclamationSystem.test.js # Achievement system tests
└── integration/           # Integration tests (future)
```

### **Naming Conventions**
- Test files: `ComponentName.test.js`
- Test suites: `describe('ComponentName', () => {})`
- Test cases: `test('should do something specific', () => {})`

## 🎯 Coverage Requirements

### **Minimum Coverage Thresholds**
- **Overall**: 80% (lines, functions, branches, statements)
- **Core Systems**: 85% (AudioSystem, ParticleSystem, etc.)
- **Critical Functions**: 100% (game state validation, configuration)

### **Coverage Commands**
```bash
npm run test:coverage     # Generate coverage report
npm run test:coverage:html # View HTML coverage report
```

## 🧩 Test Categories

### **1. Unit Tests**
Test individual functions and classes in isolation.

**Example: Configuration Validation**
```javascript
test('should have valid visual configuration', () => {
  expect(typeof CONFIG.VISUAL.GRID_SIZE).toBe('number');
  expect(CONFIG.VISUAL.GRID_SIZE).toBeGreaterThan(0);
});
```

### **2. Integration Tests**
Test interactions between multiple systems.

**Example: Audio-Game State Integration**
```javascript
test('should trigger sound effects on game events', () => {
  const audioSystem = new AudioSystem();
  expect(() => audioSystem.playEatSound()).not.toThrow();
});
```

### **3. Regression Tests**
Prevent previously fixed bugs from reoccurring.

**Example: Game State Validation**
```javascript
test('should detect invalid snake array', () => {
  const invalidState = { snake: [] };
  const validation = validateGameState(invalidState);
  expect(validation.isValid).toBe(false);
});
```

## 🔧 Mocking Strategy

### **Web Audio API Mocking**
```javascript
// tests/setup.js provides comprehensive mocks
global.AudioContext = class MockAudioContext {
  createOscillator() { /* mock implementation */ }
  createGain() { /* mock implementation */ }
};
```

### **Canvas API Mocking**
```javascript
// Canvas context mocked in setup.js
HTMLCanvasElement.prototype.getContext = jest.fn(() => ({
  fillRect: jest.fn(),
  // ... other canvas methods
}));
```

## 📝 Writing Effective Tests

### **Test Structure (AAA Pattern)**
```javascript
test('should update particle position correctly', () => {
  // Arrange
  const particle = new Particle(0, 0, 100, 0, 1000, 5);
  const initialX = particle.x;
  
  // Act
  particle.update(100);
  
  // Assert
  expect(particle.x).not.toBe(initialX);
});
```

### **Test Naming Best Practices**
- ✅ `should update particle position correctly`
- ✅ `should trigger exclamation on score milestone`
- ❌ `particle test`
- ❌ `test1`

### **Custom Matchers**
We provide custom Jest matchers for game-specific testing:

```javascript
expect(coordinates).toBeValidCoordinates(); // x,y >= 0
expect(value).toBeWithinRange(min, max);    // min <= value <= max
```

## 🎮 Testing Game-Specific Features

### **Audio System Testing**
```javascript
test('should respect mute state', () => {
  audioSystem.isMuted = true;
  const voice = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead');
  expect(voice).toBe(null);
});
```

### **Particle System Testing**
```javascript
test('should apply physics correctly', () => {
  const particle = new Particle(0, 0, 100, 0, 1000, 5);
  particle.update(100);
  expect(particle.vy).toBeGreaterThan(0); // Gravity applied
});
```

### **Configuration Testing**
```javascript
test('should validate color configuration', () => {
  const requiredColors = ['SNAKE', 'FOOD', 'BACKGROUND'];
  requiredColors.forEach(color => {
    expect(CONFIG.VISUAL.COLORS[color]).toBeDefined();
  });
});
```

## 🚨 Common Testing Pitfalls

### **❌ Avoid These Patterns**
1. **Testing Implementation Details**: Focus on behavior, not internal structure
2. **Brittle Tests**: Don't test exact values that may legitimately change
3. **Missing Edge Cases**: Test boundary conditions and error states
4. **Async Issues**: Properly handle asynchronous operations with async/await

### **✅ Best Practices**
1. **Test Behavior**: What the code should do, not how it does it
2. **Isolation**: Each test should be independent and repeatable
3. **Clarity**: Tests should serve as documentation
4. **Performance**: Keep tests fast and focused

## 🔄 Continuous Integration

### **Pre-Commit Hooks** (Future Implementation)
```bash
# Will automatically run before each commit
npm run lint       # Code quality
npm test          # Full test suite
npm run format    # Code formatting
```

### **GitHub Actions** (Future Implementation)
- Run tests on every pull request
- Generate coverage reports
- Deploy only when all tests pass

## 📊 Test Metrics

### **Current Test Coverage**
- **Total Tests**: 69 ✅
- **Passing**: 69/69 (100%) ✅
- **Coverage**: >90% (all modules) ✅

### **Test Performance**
- **Execution Time**: ~6 seconds
- **Memory Usage**: Optimized with mock objects
- **Reliability**: 100% consistent results

## 🎯 Adding Tests for New Features

### **Step-by-Step Process**

1. **Create Test File**:
   ```bash
   touch tests/NewComponent.test.js
   ```

2. **Basic Test Structure**:
   ```javascript
   import NewComponent from '../src/path/NewComponent.js';
   
   describe('NewComponent', () => {
     let component;
     
     beforeEach(() => {
       component = new NewComponent();
     });
     
     test('should initialize correctly', () => {
       expect(component).toBeDefined();
     });
   });
   ```

3. **Run Tests**:
   ```bash
   npm test -- NewComponent.test.js
   ```

4. **Add Comprehensive Coverage**:
   - Test all public methods
   - Test error conditions
   - Test edge cases
   - Test integration points

## 🏆 Testing Excellence

### **Quality Gates**
- ✅ All tests must pass
- ✅ Coverage thresholds must be met
- ✅ No console errors or warnings
- ✅ Performance requirements met

### **Review Checklist**
- [ ] Tests are clear and focused
- [ ] Edge cases are covered
- [ ] Error conditions are tested
- [ ] Performance impact is minimal
- [ ] Documentation is updated

---

**Remember: Good tests are an investment in code quality, developer productivity, and user experience. Every test we write today prevents bugs tomorrow!** 🐛→✅