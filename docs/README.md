# 📚 Goblin Camp Documentation

> *Comprehensive design documentation and technical guides for the Goblin Camp simulation game*

This directory contains the complete design documentation, architectural decisions, and development guides for the Goblin Camp project. The documentation is organized into logical sections to help developers, contributors, and users understand the system.

---

## 🚀 Quick Navigation

### 🎯 **Getting Started**
- **[Main README](../README.md)** - Project overview and quick start guide
- **[Development Roadmap](plan/MASTER_PLAN.md)** - Long-term vision and development milestones
- **[Contributing Guide](../README.md#contributing)** - How to contribute to the project

### 🏗️ **Architecture & Design**
- **[System Overview](architecture/01_overview.md)** - Core architecture and ECS design principles
- **[Architecture Decisions](architecture/adr/)** - Record of important architectural choices
- **[Project Structure](../README.md#project-structure)** - Codebase organization and module layout

### 💼 **Core Systems Documentation**

#### ⚙️ **Simulation Systems**
- **[Simulation Loop](design/sim_loop.md)** - Core game loop design and system ordering
- **[AI & Job System](design/ai_jobs.md)** - Job assignment, AI behavior, and task management
- **[Designation Lifecycle](design/designation_lifecycle.md)** - Player designation system and state management
- **[Mining, Items & Stockpiles](design/mining_items_stockpiles.md)** - Complete mining-to-storage pipeline
- **[TUI Shell Prototype](design/tui_shell.md)** - Terminal UI shell architecture, update loop, and overlays

#### 🌍 **World & Spatial Systems**
- **[World Generation](design/worldgen.md)** - Procedural world creation and terrain generation
- **[Pathfinding](design/pathfinding.md)** - A* implementation, caching, and navigation systems

#### 💾 **Data & Persistence**
- **[Save/Load System](design/save_load.md)** - Serialization, persistence, and versioning
- **[Autosave & Crash Recovery](design/autosave_crash_recovery.md)** - Rotating autosaves, recovery, and corruption handling
- **[Data Structures](../crates/gc_core/src/components.rs)** - Core ECS components and data layout

---

## 📖 Documentation Structure

```text
docs/
├── README.md                    # 📋 This file - documentation index
├── roadmap.md                   # 🗺️ Development roadmap and milestones
├── architecture/                # 🏗️ System architecture and decisions
│   ├── 01_overview.md          #     Core architecture overview
│   └── adr/                    #     Architecture Decision Records
│       ├── README.md           #     ADR index and guidelines
│       └── 0001-time-determinism.md  # Time and determinism decisions
├── design/                     # 🎨 Feature design documents
│   ├── ai_jobs.md             #     AI and job system design
│   ├── designation_lifecycle.md #     Designation state management
│   ├── mining_items_stockpiles.md #  Mining pipeline design
│   ├── pathfinding.md         #     Pathfinding implementation
│   ├── autosave_crash_recovery.md # Autosave and recovery design
│   ├── save_load.md           #     Save/load system design
│   ├── sim_loop.md            #     Simulation loop architecture
│   └── worldgen.md            #     World generation design
└── plan/                      # 📋 Development planning
    ├── MASTER_PLAN.md         #     Long-term development plan
    └── issues/                #     Issue tracking and backlogs
        └── BACKLOG_ISSUES.md  #     Development backlog
```

---

## 🔍 Documentation by Topic

### 🎯 **For New Contributors**
1. Start with **[Main README](../README.md)** for project overview
2. Read **[System Overview](architecture/01_overview.md)** for architecture understanding
3. Review **[Simulation Loop](design/sim_loop.md)** for execution flow
4. Check **[AI & Job System](design/ai_jobs.md)** for core mechanics

### 🔧 **For Core Developers**
1. **[Architecture Decisions](architecture/adr/)** - Understanding past decisions
2. **[Designation Lifecycle](design/designation_lifecycle.md)** - State management patterns
3. **[Mining Pipeline](design/mining_items_stockpiles.md)** - Complex system interactions
4. **[Save/Load System](design/save_load.md)** - Data persistence strategies

### 🎮 **For Game Designers**
1. **[AI & Job System](design/ai_jobs.md)** - Gameplay mechanics and AI behavior
2. **[World Generation](design/worldgen.md)** - Content generation systems
3. **[Master Plan](plan/MASTER_PLAN.md)** - Long-term feature roadmap

### 🏗️ **For System Architects**
1. **[System Overview](architecture/01_overview.md)** - High-level architecture
2. **[Architecture Decision Records](architecture/adr/)** - Decision rationale
3. **[Simulation Loop](design/sim_loop.md)** - System integration patterns

---

## 🌐 Online Documentation

This documentation is automatically deployed to **[GitHub Pages](https://acaradonna.github.io/goblin-camp/)** whenever changes are made to the `/docs/` directory on the main branch. The deployed version includes:

- 🎨 **Enhanced styling** with dark mode support
- 🔍 **Search functionality** across all documentation
- 📱 **Mobile-responsive design** for all devices
- 🔗 **Cross-reference linking** between documents
- 🧭 **Auto-generated table of contents** for long documents

---

## ✨ Writing Guidelines

When contributing to documentation:

### 📝 **Content Standards**
- Use clear, concise language with technical precision
- Include code examples and diagrams where helpful
- Link to relevant source code using relative paths
- Update cross-references when moving or renaming files

### 🎨 **Formatting Conventions**
- Use emojis sparingly but consistently for visual organization
- Follow markdown best practices for headers and lists
- Include `---` horizontal rules to separate major sections
- Use `> *Italics in quotes*` for emphasis and callouts

### 🔗 **Linking Best Practices**
- Use relative paths for internal documentation links
- Link to specific line numbers in source code when relevant
- Include contextual information for external links
- Test all links before committing changes

---

## 🤝 Contributing

We welcome documentation improvements! To contribute:

1. **Fork the repository** and create a feature branch
2. **Edit documentation** in your preferred markdown editor
3. **Test changes locally** by previewing markdown rendering
4. **Submit a pull request** with a clear description of changes
5. **Deployment** happens automatically once merged to main

For technical questions about the documentation system or suggestions for improvement, please [open an issue](https://github.com/acaradonna/goblin-camp/issues) with the `documentation` label.

---

*📚 Documentation maintained by the Goblin Camp development team*