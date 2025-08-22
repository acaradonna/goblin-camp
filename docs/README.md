# Goblin Camp Documentation

This directory contains the design documentation and architectural notes for the Goblin Camp project.

## 📖 Available Documentation

### 📋 Project Status
- **[Roadmap](roadmap.md)** - Development milestones, progress tracking, and release plans

### 🏗️ Architecture
- **[System Overview](architecture/01_overview.md)** - Core architecture and ECS design principles

### 🎨 Design Documents
- **[AI & Job System](design/ai_jobs.md)** - Job assignment, AI behavior, and designation system
- **[Designation Lifecycle](design/designation_lifecycle.md)** - Designation state management and deduplication system
- **[Pathfinding](design/pathfinding.md)** - A* implementation, caching, and navigation systems
- **[Save/Load System](design/save_load.md)** - Serialization, persistence, and versioning
- **[Simulation Loop](design/sim_loop.md)** - Core game loop design and system ordering
- **[World Generation](design/worldgen.md)** - Procedural world creation and terrain generation

## 🌐 Online Documentation

This documentation is automatically deployed to GitHub Pages whenever changes are made to the `/docs/` directory on the main branch. The deployed version includes enhanced navigation and styling for easy browsing.

## 📝 Contributing to Documentation

When updating documentation:
1. Edit the Markdown files in their respective directories
2. Follow the existing structure and formatting conventions
3. Commit changes to the main branch to trigger automatic deployment
4. The GitHub Pages site will be updated automatically within a few minutes

## 🔧 Technical Implementation

The documentation deployment uses:
- **GitHub Actions** for automatic builds and deployment
- **Pandoc** for Markdown to HTML conversion
- **GitHub Pages** for hosting the static site
- **GitHub-flavored styling** for consistent appearance