---
layout: default
title: Goblin Camp Documentation
description: Design documentation and technical guides for the Goblin Camp simulation game
---

<!-- markdownlint-disable MD033 MD031 MD022 MD036 MD025 -->

# 🏰 Goblin Camp Documentation

> *Comprehensive design documentation and technical guides for the Goblin Camp simulation game*

Welcome to the Goblin Camp documentation! This site contains detailed information about the architecture, design decisions, and technical implementation of our colony simulation game.

## 🚀 Quick Navigation

<div class="nav-grid">
  <div class="nav-card">
    <h3>🎯 Getting Started</h3>
    <ul>
      <li><a href="{{ site.github.repository_url }}">Main Repository</a></li>
      <li><a href="/plan/MASTER_PLAN">Development Roadmap</a></li>
      <li><a href="/roadmap">Current Milestones</a></li>
    </ul>
  </div>

  <div class="nav-card">
    <h3>🏗️ Architecture</h3>
    <ul>
      <li><a href="/architecture/01_overview">System Overview</a></li>
      <li><a href="/architecture/adr/">Architecture Decisions</a></li>
    </ul>
  </div>

  <div class="nav-card">
    <h3>🎨 Design Documents</h3>
    <ul>
      <li><a href="/design/ai_jobs">AI & Job System</a></li>
      <li><a href="/design/designation_lifecycle">Designation Lifecycle</a></li>
      <li><a href="/design/mining_items_stockpiles">Mining & Stockpiles</a></li>
  <li><a href="/design/zones_stockpiles_rules">Zones & Stockpile Rules</a></li>
  <li><a href="/design/fluids_2d_temperature">Fluids & Temperature</a></li>
      <li><a href="/design/pathfinding">Pathfinding</a></li>
      <li><a href="/design/save_load">Save/Load System</a></li>
      <li><a href="/design/sim_loop">Simulation Loop</a></li>
      <li><a href="/design/worldgen">World Generation</a></li>
    </ul>
  </div>
</div>

## 🎬 Live Demos

Explore the current functionality through our interactive CLI demos:

### 🗺️ Map Generation Demo

```bash
cargo run -p gc_cli -- mapgen
```

*Procedural terrain generation with noise-based height maps*

### ⛏️ Mining & Hauling Pipeline

```bash
cargo run -p gc_cli -- jobs
```

*Complete workflow from mining walls to stockpiling items*

### 🎯 Pathfinding Visualization

```bash
cargo run -p gc_cli -- path
```

*A* algorithm with LRU caching and path visualization*

### 👁️ Field of View System

```bash
cargo run -p gc_cli -- --show-vis fov
```

*Line-of-sight calculations with visibility overlay*

## 🎯 Current Features

- **🗺️ Procedural World Generation** - Noise-based terrain with configurable parameters
- **🔍 Optimized Pathfinding** - A* algorithm with LRU caching for performance
- **💼 Hierarchical Job System** - Mining, hauling, and task assignment with AI behavior
- **📦 Spatial Item Management** - Full ECS entities with inventory and stockpile systems
- **👁️ Field of View** - Line-of-sight calculations for visibility mechanics
- **💾 Save/Load System** - JSON serialization with versioning support
- **🎮 Interactive CLI** - Comprehensive demo system for testing and visualization

## 🧪 Development Status

This project is under active development with a focus on building robust simulation systems. Check our [roadmap](/roadmap) for current milestones and [architecture docs](/architecture/) for technical details.

## 🤝 Contributing

We welcome contributions! See our [contribution guidelines]({{ site.github.repository_url }}#contributing) for how to get started.

---

*📚 Documentation automatically deployed from the [main repository]({{ site.github.repository_url }})*

<style>
.nav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 2rem 0;
}

.nav-card {
  background: var(--toc-bg, #f8f9fa);
  border: 1px solid var(--toc-border, #e9ecef);
  border-radius: 8px;
  padding: 1.5rem;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.nav-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.nav-card h3 {
  margin-top: 0;
  color: var(--toc-heading, #333);
  border-bottom: 2px solid #159957;
  padding-bottom: 0.5rem;
}

.nav-card ul {
  list-style: none;
  padding: 0;
  margin: 1rem 0 0 0;
}

.nav-card li {
  margin: 0.75rem 0;
}

.nav-card a {
  color: var(--toc-link, #159957);
  text-decoration: none;
  font-weight: 500;
}

.nav-card a:hover {
  text-decoration: underline;
}

.dark-mode .nav-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}
</style>
