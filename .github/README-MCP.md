# MCP Servers for Goblin Camp

This directory contains Model Context Protocol (MCP) server configurations optimized for developing the Goblin Camp project with GitHub Copilot.

## Quick Start

1. **Install MCP servers:**
   ```bash
   ./setup-mcp.sh
   ```

2. **Set environment variables:**
   ```bash
   export GITHUB_TOKEN="your-github-token"
   export BRAVE_API_KEY="your-brave-api-key"  # optional
   ```

3. **Choose a configuration:**
   - `mcp-config-minimal.json` - Essential servers only
   - `mcp-config-enhanced.json` - Recommended for full development
   - `mcp-servers.json` - All available servers

4. **Copy to your GitHub Copilot client settings**

## Configurations

### Minimal (4 servers)
Essential for basic development:
- **filesystem** - Navigate project files
- **github** - Repository management  
- **git** - Version control
- **memory** - Conversation context

### Enhanced (8 servers)
Recommended for full development experience:
- All minimal servers plus:
- **sqlite** - Database for save system
- **brave-search** - Research capabilities
- **time** - Scheduling and timestamps
- **sequential-thinking** - Complex problem solving

## Benefits for Goblin Camp

- 🦀 **Rust Development**: Navigate multi-crate workspace efficiently
- 🎮 **Game Development**: SQLite for save system, research for game mechanics
- 📊 **Project Management**: Track M0-M3 milestones, manage issues/PRs
- 📚 **Documentation**: Access extensive docs/ directory seamlessly
- 🔍 **Research**: Find Rust/Bevy/game development resources

## Documentation

See `mcp-servers.md` for detailed server descriptions and usage examples.