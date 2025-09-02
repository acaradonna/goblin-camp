# 🤖 Agent Workspace Configuration

This file configures the optimal development environment for AI agents working on the Goblin Camp project.

## 🎯 Project Context

**Goblin Camp** is a Rust-based colony simulation game inspired by Dwarf Fortress, featuring:
- ECS architecture with Bevy
- Procedural world generation
- Complex simulation systems (jobs, pathfinding, FOV)
- Comprehensive test suite and documentation

## 🚀 Agent Mode Optimizations

### **VS Code Configuration**
- **Enhanced Rust Analyzer**: Full Rust language support with optimizations
- **Comprehensive Tasks**: 20+ pre-configured development commands
- **Debug Configurations**: LLDB debugging for CLI, core, and TUI
- **Performance Settings**: Optimized file watching and search exclusions

### **Development Shortcuts**
- **Quick Commands**: `Ctrl+Shift+P` → "Tasks: Run Task" → Select GC task
- **Fast Validation**: `GC: Cargo Check` for syntax validation
- **Quality Checks**: `GC: Lint (Clippy)` for code quality
- **Demo Testing**: Direct access to all CLI demos

## 🔧 Agent Development Workflow

### **1. Code Changes**
```rust
// Make changes in Rust files
// Use Ctrl+Shift+P → "GC: Cargo Check" for quick validation
// Use Ctrl+Shift+P → "GC: Format Code" for formatting
```

### **2. Testing & Validation**
```bash
# Quick tests: Ctrl+Shift+P → "GC: Fast Tests"
# Full tests: Ctrl+Shift+P → "GC: Test Suite"
# Quality check: Ctrl+Shift+P → "GC: Full Validation"
```

### **3. Demo & Debug**
```bash
# Test features: Ctrl+Shift+P → "GC: CLI [Demo]"
# Debug: Use launch configurations in Debug panel
```

## 📚 Key Documentation

### **Design Documents** (Essential for agents)
- `docs/design/worldgen.md` - World generation system (Epic #37)
- `docs/design/combat_mvp.md` - Combat MVP implementation
- `docs/design/fluids_2d_temperature.md` - Fluid simulation
- `docs/roadmap.md` - Development milestones and phases

### **Architecture**
- `docs/architecture/` - System architecture decisions
- `docs/plan/` - Development plans and specifications

## 🎮 Current Development Status

### **M3 Milestone (Active)**
- **Combat MVP**: Core combat systems, health, factions
- **Fluids**: 2D cellular automata simulation
- **TUI Shell**: Text-based interface prototype

### **Worldgen Epic #37 (Planned)**
- 9-part epic for procedural world generation
- Biomes, civilizations, embark sites
- Heightmap, temperature, rainfall systems

## 🔍 Agent-Specific Commands

### **Quick Development**
- `GC: Quick Setup` - Full environment setup
- `GC: Cargo Check` - Fast syntax validation
- `GC: Format Code` - Auto-formatting
- `GC: Lint (Clippy)` - Code quality

### **Testing & Validation**
- `GC: Fast Tests` - Unit tests only
- `GC: Test Suite` - Full test suite
- `GC: Full Validation` - Complete check

### **Feature Demos**
- `GC: CLI Mapgen Demo` - World generation
- `GC: CLI Jobs Demo` - Job system
- `GC: CLI Path Demo` - Pathfinding
- `GC: CLI FOV Demo` - Field of view

### **Git & GitHub**
- `GC: Git Status` - Repository status
- `GC: GitHub Issues` - Recent issues

## 🚨 Agent Mode Best Practices

### **1. Use VS Code Tasks**
- Prefer VS Code tasks over terminal commands
- Tasks provide consistent output and error handling
- Use `Ctrl+Shift+P` → "Tasks: Run Task" for quick access

### **2. Leverage Rust Analyzer**
- Hover over code for documentation
- Use "Go to Definition" for navigation
- Leverage "Show References" for understanding usage

### **3. Check Design Docs First**
- Always review design documents before implementing
- Understand the system architecture and constraints
- Follow established patterns and conventions

### **4. Use Incremental Development**
- Start with `cargo check` for quick validation
- Use fast tests for rapid iteration
- Run full validation before committing

## 🔧 Environment Variables

### **Development**
- `RUST_BACKTRACE=1` - Full backtraces for debugging
- `RUST_LOG=info` - Detailed logging output
- `CARGO_INCREMENTAL=1` - Incremental compilation

### **Performance**
- `CARGO_NET_RETRY=10` - Network resilience
- `CARGO_NET_TIMEOUT=60` - Network timeout
- `CARGO_TERM_COLOR=always` - Colored output

## 📋 Agent Task Checklist

### **Before Starting Development**
- [ ] Review relevant design documents
- [ ] Understand current system architecture
- [ ] Check existing test coverage
- [ ] Review related GitHub issues

### **During Development**
- [ ] Use `GC: Cargo Check` for quick validation
- [ ] Run `GC: Fast Tests` for unit tests
- [ ] Use `GC: Format Code` for consistency
- [ ] Test with relevant CLI demos

### **Before Committing**
- [ ] Run `GC: Full Validation`
- [ ] Ensure all tests pass
- [ ] Check code formatting
- [ ] Update documentation if needed

## 🎯 Performance Tips

### **Fast Iteration**
- Use `cargo check` instead of `cargo build`
- Leverage incremental compilation
- Use fast tests for quick validation
- Prefer VS Code tasks over terminal commands

### **Memory Management**
- Watch for memory leaks in long-running simulations
- Use appropriate data structures for performance
- Leverage Rust's ownership system for efficiency

---

**Agent Mode Ready! 🚀** This workspace is optimized for efficient AI agent development with comprehensive tooling, shortcuts, and best practices.
