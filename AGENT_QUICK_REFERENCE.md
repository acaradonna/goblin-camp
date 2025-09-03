# 🚀 Agent Quick Reference Card

## ⚡ **FAST COMMANDS** (Ctrl+Shift+P → Tasks: Run Task)

### **Development Cycle**
- `GC: Cargo Check` → Quick syntax validation
- `GC: Format Code` → Auto-format with rustfmt  
- `GC: Lint (Clippy)` → Code quality check
- `GC: Fast Tests` → Unit tests only (fast)

### **Feature Testing**
- `GC: CLI Mapgen Demo` → World generation
- `GC: CLI Jobs Demo` → Job system
- `GC: CLI Path Demo` → Pathfinding
- `GC: CLI FOV Demo` → Field of view

### **Full Validation**
- `GC: Full Validation` → Format + Clippy + Tests
- `GC: Test Suite` → Complete test suite
- `GC: Quick Setup` → Environment setup

## 🔍 **QUICK NAVIGATION**

### **Key Files**
- `docs/design/worldgen.md` → World generation design
- `docs/design/combat_mvp.md` → Combat system design
- `docs/roadmap.md` → Development roadmap
- `.claude/agent-workspace.md` → Agent configuration

### **Project Structure**
- `crates/gc_core/` → Core engine (ECS, simulation)
- `crates/gc_cli/` → Command-line interface
- `crates/gc_tui/` → Text-based interface

## 🎯 **CURRENT FOCUS**

### **M3 Milestone (Active)**
- Combat MVP: Health, factions, deterministic combat
- Fluids: 2D cellular automata simulation
- TUI Shell: Text-based interface prototype

### **Worldgen Epic #37 (Planned)**
- 9-part epic for procedural world generation
- Biomes, civilizations, embark sites

## 🚨 **AGENT BEST PRACTICES**

1. **Use VS Code Tasks** instead of terminal commands
2. **Check design docs first** before implementing
3. **Leverage Rust Analyzer** for navigation and hints
4. **Use incremental development** with fast validation

## 🔧 **ENVIRONMENT**

- **Rust 1.81+** with Bevy ECS
- **Comprehensive test suite** with benchmarks
- **GitHub CLI** for issue management
- **VS Code** with Rust Analyzer optimization

---

**Need help?** Check `.vscode/agent-mode.md` for full guide!
