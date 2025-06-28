module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
    jest: true
  },
  extends: [
    'eslint:recommended',
    'prettier'
  ],
  plugins: [
    'prettier'
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  rules: {
    // Prettier integration
    'prettier/prettier': 'error',
    
    // Code quality
    'no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    
    // Best practices
    'eqeqeq': ['error', 'always'],
    'curly': ['error', 'all'],
    'no-eval': 'error',
    'no-implied-eval': 'error',
    'no-new-wrappers': 'error',
    'no-throw-literal': 'error',
    'prefer-const': 'error',
    'no-var': 'error',
    
    // Style (handled by Prettier but good to enforce)
    'indent': 'off', // Handled by Prettier
    'quotes': ['error', 'single', { avoidEscape: true }],
    'semi': ['error', 'always'],
    'comma-dangle': ['error', 'never'],
    
    // ES6+
    'arrow-spacing': 'error',
    'no-duplicate-imports': 'error',
    'object-shorthand': 'error',
    'prefer-arrow-callback': 'error',
    'prefer-template': 'error',
    
    // Game-specific rules
    'no-magic-numbers': ['warn', { 
      ignore: [0, 1, -1, 2, 60, 1000],
      ignoreArrayIndexes: true,
      ignoreDefaultValues: true
    }],
    'max-len': ['warn', { 
      code: 100, 
      ignoreComments: true,
      ignoreStrings: true,
      ignoreTemplateLiterals: true
    }],
    'max-complexity': ['warn', 10],
    'max-depth': ['warn', 4]
  },
  overrides: [
    {
      files: ['**/*.test.js', 'tests/**/*.js'],
      rules: {
        'no-magic-numbers': 'off',
        'max-len': 'off'
      }
    },
    {
      files: ['webpack.config.js', '.eslintrc.js'],
      env: {
        node: true,
        browser: false
      },
      rules: {
        'no-console': 'off'
      }
    }
  ],
  globals: {
    // Game globals
    'CONFIG': 'readonly',
    'gameState': 'writable',
    'AudioContext': 'readonly',
    'webkitAudioContext': 'readonly',
    
    // Testing globals
    'describe': 'readonly',
    'it': 'readonly',
    'expect': 'readonly',
    'beforeEach': 'readonly',
    'afterEach': 'readonly',
    'beforeAll': 'readonly',
    'afterAll': 'readonly',
    'jest': 'readonly'
  }
};