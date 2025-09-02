#!/bin/bash
# Agent-optimized development script for Goblin Camp

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[AGENT]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

case "$1" in
    "quick-check"|"qc")
        print_status "Quick syntax check..."
        cargo check
        print_success "Syntax check passed!"
        ;;

    "fast-test"|"ft")
        print_status "Running fast tests (unit tests only)..."
        if command_exists cargo-nextest; then
            cargo nextest run --lib
        else
            cargo test --lib
        fi
        print_success "Fast tests completed!"
        ;;

    "format"|"fmt")
        print_status "Formatting code..."
        cargo fmt --all
        print_success "Code formatted!"
        ;;

    "lint"|"clippy")
        print_status "Running Clippy linter..."
        cargo clippy --workspace --all-targets --all-features
        print_success "Linting passed!"
        ;;

    "validate"|"val")
        print_status "Running full validation..."
        print_status "Step 1/3: Format check..."
        cargo fmt --all -- --check
        print_status "Step 2/3: Clippy lint..."
        cargo clippy --workspace --all-targets --all-features
        print_status "Step 3/3: Test suite..."
        if command_exists cargo-nextest; then
            cargo nextest run --workspace
        else
            cargo test --workspace
        fi
        print_success "Full validation completed!"
        ;;

    "demo"|"test-features")
        print_status "Testing current features..."
        echo "Available demos:"
        echo "  mapgen  - World generation"
        echo "  jobs    - Job system"
        echo "  path    - Pathfinding"
        echo "  fov     - Field of view"
        echo "  save    - Save/load system"
        echo ""
        echo "Usage: ./agent-dev.sh demo <demo-name>"
        if [ -n "$2" ]; then
            print_status "Running demo: $2"
            cargo run -p gc_cli -- "$2"
        fi
        ;;

    "status"|"st")
        print_status "Project status..."
        echo ""
        echo "Git Status:"
        git status --short
        echo ""
        echo "Recent Commits:"
        git log --oneline -5
        echo ""
        echo "Open Issues:"
        gh issue list --limit 5 --json number,title,state,labels
        ;;

    "setup"|"init")
        print_status "Setting up agent development environment..."
        print_status "Installing recommended tools..."

        # Check for cargo-nextest
        if ! command_exists cargo-nextest; then
            print_warning "cargo-nextest not found. Installing..."
            cargo install cargo-nextest
        fi

        # Check for GitHub CLI
        if ! command_exists gh; then
            print_warning "GitHub CLI not found. Please install manually."
        fi

        print_status "Building project..."
        cargo build

        print_status "Running tests..."
        if command_exists cargo-nextest; then
            cargo nextest run
        else
            cargo test
        fi

        print_success "Agent environment setup complete!"
        print_status "Use './agent-dev.sh quick-check' for fast validation"
        print_status "Use './agent-dev.sh validate' for full validation"
        ;;

    "help"|"h"|"")
        echo "🚀 Agent Development Script for Goblin Camp"
        echo ""
        echo "Quick Commands:"
        echo "  qc, quick-check    - Fast syntax check"
        echo "  ft, fast-test      - Unit tests only"
        echo "  fmt, format        - Format code"
        echo "  clippy, lint       - Run Clippy"
        echo "  val, validate      - Full validation"
        echo "  demo <name>        - Test features"
        echo "  st, status         - Project status"
        echo "  init, setup        - Setup environment"
        echo ""
        echo "Examples:"
        echo "  ./agent-dev.sh qc           # Quick check"
        echo "  ./agent-dev.sh ft           # Fast tests"
        echo "  ./agent-dev.sh demo mapgen  # Test worldgen"
        echo "  ./agent-dev.sh validate     # Full validation"
        echo ""
        echo "VS Code Tasks: Use Ctrl+Shift+P → 'Tasks: Run Task' for GUI access"
        ;;

    *)
        print_error "Unknown command: $1"
        echo "Use './agent-dev.sh help' for available commands"
        exit 1
        ;;
esac
