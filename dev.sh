#!/bin/bash
# Development helper script for Goblin Camp

set -e

case "$1" in
    "setup"|"")
        echo "Setting up Goblin Camp development environment..."
        echo "Building project..."
        cargo build
        echo "Running tests..."
        cargo test
        echo "✓ Setup complete! Try: ./dev.sh demo"
        ;;
    "test")
        echo "Running tests..."
        cargo test
        ;;
    "lint")
        echo "Running clippy linter..."
        cargo clippy
        ;;
    "format")
        echo "Formatting code..."
        cargo fmt
        ;;
    "check")
        echo "Running full checks (format, lint, test)..."
        cargo fmt --check || (echo "Code needs formatting. Run: ./dev.sh format" && exit 1)
        cargo clippy
        cargo test
        echo "✓ All checks passed!"
        ;;
    "demo")
        echo "Running interactive demo menu..."
        cargo run -p gc_cli -- menu
        ;;
    "build")
        echo "Building project..."
        cargo build
        ;;
    "help")
        echo "Goblin Camp development script"
        echo ""
        echo "Usage: ./dev.sh [command]"
        echo ""
        echo "Commands:"
        echo "  setup     Setup development environment (default)"
        echo "  build     Build the project"
        echo "  test      Run all tests"
        echo "  lint      Run clippy linter"
        echo "  format    Format code with rustfmt"
        echo "  check     Run format check, lint, and tests"
        echo "  demo      Run interactive demo menu"
        echo "  help      Show this help message"
        echo ""
        echo "Examples:"
        echo "  ./dev.sh              # Setup environment"
        echo "  ./dev.sh test         # Run tests"
        echo "  ./dev.sh check        # Full validation"
        echo "  ./dev.sh demo         # Try the demos"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Run './dev.sh help' for available commands"
        exit 1
        ;;
esac