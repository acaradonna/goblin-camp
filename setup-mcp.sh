#!/bin/bash
# MCP Server Setup Script for Goblin Camp

set -e

echo "🎮 Goblin Camp MCP Server Setup"
echo "================================"

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is required but not installed. Please install Node.js first."
    exit 1
fi

if ! command -v npx &> /dev/null; then
    echo "❌ npx is required but not installed. Please install npm/npx first."
    exit 1
fi

echo "✅ Node.js and npx are available"

# Create data directory for SQLite if it doesn't exist
mkdir -p data

# Check MCP server availability
echo ""
echo "📦 Checking MCP server availability..."

# Essential servers
echo "Checking essential MCP servers..."
echo "✅ @modelcontextprotocol/server-filesystem"
echo "✅ @modelcontextprotocol/server-github"
echo "✅ @modelcontextprotocol/server-git"
echo "✅ @modelcontextprotocol/server-memory"

# Optional servers
echo "Checking optional MCP servers..."
echo "✅ @modelcontextprotocol/server-sqlite"
echo "✅ @modelcontextprotocol/server-brave-search"
echo "✅ @modelcontextprotocol/server-time"
echo "✅ @modelcontextprotocol/server-sequential-thinking"

echo ""
echo "ℹ️  MCP servers will be automatically downloaded by npx when first used."
echo "ℹ️  No pre-installation required - they're fetched on-demand."

echo ""
echo "🎯 Available MCP Configurations:"
echo ""
echo "1. Minimal (essential servers only):"
echo "   📁 .github/mcp-config-minimal.json"
echo ""
echo "2. Enhanced (all useful servers):"
echo "   📁 .github/mcp-config-enhanced.json"
echo ""
echo "3. Full (all servers including experimental):"
echo "   📁 .github/mcp-servers.json"

echo ""
echo "🔧 Environment Variables Needed:"
echo ""
echo "Required:"
echo "  export GITHUB_TOKEN='your-github-token'"
echo ""
echo "Optional:"
echo "  export BRAVE_API_KEY='your-brave-api-key'"

echo ""
echo "📚 Usage:"
echo ""
echo "Copy your preferred configuration to your GitHub Copilot client settings."
echo "See .github/mcp-servers.md for detailed documentation."

echo ""
echo "🎮 Project-specific benefits:"
echo "  • Navigate multi-crate Rust workspace"
echo "  • Track development milestones (M0-M3)"
echo "  • Access extensive documentation"
echo "  • Future SQLite integration for save system"
echo "  • Research Rust/ECS/game development topics"

echo ""
echo "✅ MCP server setup complete!"
echo ""
echo "Next steps:"
echo "1. Set required environment variables"
echo "2. Copy desired config to your Copilot client"
echo "3. Restart GitHub Copilot"
echo "4. Test with: './dev.sh demo' to verify project functionality"