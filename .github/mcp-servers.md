# MCP Server Configuration for Goblin Camp

This document describes the Model Context Protocol (MCP) servers configured for GitHub Copilot to enhance development on the Goblin Camp project.

## Configured MCP Servers

### 1. File System Server (`filesystem`)
**Purpose**: Navigate and manipulate files in the workspace
**Benefits for Goblin Camp**:
- Browse the multi-crate workspace structure (`crates/gc_core`, `crates/gc_cli`) 
- Access documentation files in `docs/` directory
- Read and modify Rust source files
- Access configuration files (Cargo.toml, dev.sh)

**Configuration**:
- Allowed directory: `/home/runner/work/goblin-camp/goblin-camp`
- Provides secure file system access within the project root

### 2. GitHub Server (`github`)
**Purpose**: Interact with GitHub API for repository management
**Benefits for Goblin Camp**:
- Manage issues and pull requests
- Track milestone progress (M0-M3 roadmap)
- Analyze commit history and code changes
- Review and create releases
- Manage project workflows and actions

**Configuration**:
- Uses GITHUB_TOKEN environment variable for authentication
- Connects to GitHub API at https://api.github.com

### 3. Git Server (`git`)
**Purpose**: Perform Git version control operations
**Benefits for Goblin Camp**:
- Analyze commit history and branch structure
- Check file change history for debugging
- Manage local Git operations
- Track development progress across milestones

**Configuration**:
- Repository path: `/home/runner/work/goblin-camp/goblin-camp`
- Provides local Git operations complementing GitHub server

### 4. SQLite Server (`sqlite`)
**Purpose**: Database operations for potential game data storage
**Benefits for Goblin Camp**:
- **Future feature**: Enhanced save/load system using SQLite
- Store game state more efficiently than JSON
- Query and analyze saved game data
- Performance optimization for large game worlds

**Configuration**:
- Database path: `/home/runner/work/goblin-camp/goblin-camp/data/saves.db`
- Ready for when the project migrates from JSON to database storage

### 5. Brave Search Server (`brave-search`)
**Purpose**: Search the web for development resources
**Benefits for Goblin Camp**:
- Research Rust/Bevy ECS patterns and best practices
- Find solutions for game development challenges
- Look up Dwarf Fortress mechanics for inspiration
- Search for pathfinding and simulation algorithms

**Configuration**:
- Uses BRAVE_API_KEY environment variable
- Provides web search capabilities for research

### 6. Time Server (`time`)
**Purpose**: Handle time-related operations and scheduling
**Benefits for Goblin Camp**:
- Track development milestones and deadlines
- Generate timestamps for save files
- Schedule automated tasks and builds
- Time-based game simulation features

**Configuration**:
- No special configuration required
- Provides current time and date utilities

### 7. Memory Server (`memory`)
**Purpose**: Persistent conversation memory across sessions
**Benefits for Goblin Camp**:
- Remember project context and decisions
- Track ongoing development discussions
- Maintain context about architectural choices
- Remember specific implementation details

**Configuration**:
- No special configuration required
- Stores conversation context persistently

## Usage Examples

### Development Workflow
```bash
# File system server: Browse project structure
# Git server: Check recent changes
# GitHub server: Review current issues and PRs
# Memory server: Remember previous architecture discussions
```

### Research and Planning
```bash
# Brave search server: Research ECS patterns for colony simulation
# Time server: Plan milestone deadlines
# Memory server: Track design decisions
```

### Game Development
```bash
# SQLite server: Design enhanced save system
# File system server: Modify simulation code
# Git server: Track changes and versions
```

## Environment Variables Required

To use these MCP servers, ensure the following environment variables are set:

```bash
# GitHub integration
export GITHUB_TOKEN="your-github-token"

# Web search (optional)
export BRAVE_API_KEY="your-brave-api-key"
```

## Project-Specific Benefits

### For Rust Development
- **File system server**: Navigate complex multi-crate workspace
- **Git server**: Track changes across multiple Rust files
- **GitHub server**: Manage Rust-specific CI/CD workflows

### For Game Development
- **SQLite server**: Future-ready for game database needs
- **Time server**: Game simulation and timing features
- **Memory server**: Remember game design decisions

### For Documentation
- **File system server**: Access extensive docs/ directory
- **GitHub server**: Manage documentation in issues/PRs
- **Brave search server**: Research game development topics

### For Project Management
- **GitHub server**: Track M0-M3 milestone progress
- **Time server**: Development scheduling
- **Memory server**: Maintain project context

## Security Considerations

- File system access is restricted to the project directory
- GitHub token should have minimal required permissions
- SQLite database is local to the project
- All servers operate within the sandboxed environment

## Future Expansions

As the project grows, consider adding:
- **Web server**: For potential web-based game interface
- **Postgres server**: If moving to more complex database needs
- **Sequential thinking server**: For complex AI planning systems
- **Fetch server**: For external content integration