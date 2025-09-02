# 🚀 Agent Mode Development Guide

This workspace has been optimized for AI agent development with comprehensive tooling and shortcuts.

## 🎯 Quick Start Commands

### **Build & Test (Ctrl+Shift+P → Tasks: Run Task)**
- `GC: Quick Setup` - Full project setup and validation
- `GC: Full Validation` - Format + Clippy + Tests
- `GC: Test Suite` - Run all tests
- `GC: Fast Tests` - Unit tests only (faster)

### **Development Demos**
- `GC: CLI Mapgen Demo` - World generation demo
- `GC: CLI Jobs Demo` - Job system demo
- `GC: CLI Path Demo` - Pathfinding demo
- `GC: CLI FOV Demo` - Field of view demo

### **Code Quality**
- `GC: Format Code` - Auto-format with rustfmt
- `GC: Lint (Clippy)` - Run Clippy linter
- `GC: Cargo Check` - Quick syntax check

### **Git & GitHub**
- `GC: Git Status` - Check repository status
- `GC: GitHub Issues` - View recent issues

## 🔧 Keyboard Shortcuts

### **Rust Development**
- `Ctrl+Shift+P` → "Rust Analyzer: Reload Workspace"
- `Ctrl+Shift+P` → "Rust Analyzer: Show References"
- `Ctrl+Shift+P` → "Rust Analyzer: Go to Implementation"

### **Tasks & Commands**
- `Ctrl+Shift+P` → "Tasks: Run Task" → Select GC task
- `Ctrl+Shift+P` → "Developer: Reload Window" (if needed)

## 🏗️ Project Structure

```
goblin-camp/
├── crates/
│   ├── gc_core/     # Core engine (ECS, simulation, jobs)
│   ├── gc_cli/      # Command-line interface & demos
│   └── gc_tui/      # Text-based user interface
├── docs/            # Comprehensive design documentation
├── scripts/         # Development utilities
└── .vscode/         # Agent-optimized VS Code config
```

## 🎮 Current Features

### **✅ Implemented**
- **World Generation**: Procedural terrain with noise
- **Pathfinding**: A* algorithm with LRU caching
- **Job System**: Mining, hauling, task assignment
- **Field of View**: Line-of-sight calculations
- **Save/Load**: JSON serialization with versioning
- **CLI Demos**: Interactive testing system

### **🚧 In Development (M3)**
- **Combat MVP**: Health, factions, deterministic combat
- **Fluids**: 2D cellular automata simulation
- **TUI Shell**: Text-based interface prototype

### **📋 Planned (Worldgen Epic #37)**
- **Overworld Generation**: Biomes, civilizations, embark sites
- **Advanced Systems**: Z-levels, AI, modding support

## 🚀 Development Workflow

### **1. Quick Development Cycle**
```bash
# Make changes, then:
Ctrl+Shift+P → "GC: Cargo Check"     # Quick syntax check
Ctrl+Shift+P → "GC: Format Code"     # Auto-format
Ctrl+Shift+P → "GC: Lint (Clippy)"   # Code quality
Ctrl+Shift+P → "GC: Test Suite"      # Run tests
```

### **2. Demo Testing**
```bash
# Test current functionality:
Ctrl+Shift+P → "GC: CLI Mapgen Demo"  # World generation
Ctrl+Shift+P → "GC: CLI Jobs Demo"    # Job system
Ctrl+Shift+P → "GC: CLI Path Demo"    # Pathfinding
```

### **3. Full Validation**
```bash
# Before committing:
Ctrl+Shift+P → "GC: Full Validation"  # Complete check
```

## 🔍 Debugging

### **Launch Configurations**
- `GC: Debug CLI` - Debug CLI application
- `GC: Debug Core Tests` - Debug core engine tests
- `GC: Debug TUI` - Debug text interface

### **Environment Variables**
- `RUST_BACKTRACE=1` - Full backtraces
- `RUST_LOG=debug` - Detailed logging

## 📚 Documentation

### **Design Documents**
- `/docs/design/worldgen.md` - World generation system
- `/docs/design/combat_mvp.md` - Combat system design
- `/docs/design/fluids_2d_temperature.md` - Fluid simulation
- `/docs/roadmap.md` - Development roadmap

### **Architecture**
- `/docs/architecture/` - System architecture
- `/docs/plan/` - Development plans

## 🎯 Agent Mode Tips

### **Efficient Development**
1. Use VS Code tasks instead of terminal commands
2. Leverage Rust Analyzer features (hover, go-to-definition)
3. Use the comprehensive test suite for validation
4. Check design docs before implementing features

### **Common Commands**
- **Quick check**: `Ctrl+Shift+P` → "GC: Cargo Check"
- **Run tests**: `Ctrl+Shift+P` → "GC: Test Suite"
- **Format code**: `Ctrl+Shift+P` → "GC: Format Code"
- **View issues**: `Ctrl+Shift+P` → "GC: GitHub Issues"

### **Performance**
- Use `GC: Fast Tests` for quick validation
- Leverage incremental compilation (`cargo check`)
- Use the dev script for optimized builds

## 🚨 Troubleshooting

### **Common Issues**
- **Rust Analyzer not working**: Reload workspace
- **Build errors**: Run `GC: Quick Setup`
- **Test failures**: Check `GC: Test Suite` output
- **Format issues**: Run `GC: Format Code`

### **Reset Environment**
```bash
Ctrl+Shift+P → "Developer: Reload Window"
# or
Ctrl+Shift+P → "GC: Quick Setup"
```

---

**Happy coding! 🎉** This workspace is now optimized for efficient AI agent development.
