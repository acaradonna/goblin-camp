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
    "coverage")
        echo "Generating code coverage report..."
        echo "Installing cargo-llvm-cov if not present..."
        cargo install cargo-llvm-cov --quiet || true
        echo "Generating HTML coverage report (core library only)..."
        cargo llvm-cov --html --output-dir target/coverage --package gc_core
        echo "Generating LCOV report for external tools..."
        cargo llvm-cov --lcov --output-path target/coverage/lcov.info --package gc_core
        echo "✓ Coverage reports generated in target/coverage/"
        echo "  - HTML report: target/coverage/html/index.html"
        echo "  - LCOV report: target/coverage/lcov.info"
        echo "  - Core library only (industry standard for UI code exclusion)"
        ;;
    "coverage-check")
        echo "Running coverage with threshold enforcement..."
        cargo install cargo-llvm-cov --quiet || true
        # Set minimum coverage threshold to 75% for core library (excludes CLI UI code)
        # This follows industry standard practice of excluding UI/CLI from coverage
        cargo llvm-cov --fail-under-lines 75 --summary-only --package gc_core
        if [ $? -eq 0 ]; then
            echo "✓ Core library coverage meets minimum threshold (75%)"
            echo "  CLI interface excluded per industry standards"
        else
            echo "❌ Core library coverage below minimum threshold (75%)"
            echo "Run './dev.sh coverage' to see detailed coverage report"
            exit 1
        fi
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
        echo "  check     Run format check, lint, and tests (matches CI validation)"
        echo "  coverage  Generate code coverage reports (HTML + LCOV)"
        echo "  coverage-check  Run coverage with minimum threshold enforcement"
        echo "  demo      Run interactive demo menu"
        echo "  help      Show this help message"
        echo ""
        echo "Examples:"
        echo "  ./dev.sh              # Setup environment"
        echo "  ./dev.sh test         # Run tests"
        echo "  ./dev.sh check        # Full validation (same as CI)"
        echo "  ./dev.sh coverage     # Generate coverage reports (core library)"
        echo "  ./dev.sh coverage-check  # Check coverage meets threshold (75% core)"
        echo "  ./dev.sh demo         # Try the demos"
        echo ""
        echo "Note: All PRs are automatically validated by CI with the same checks as './dev.sh check'"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Run './dev.sh help' for available commands"
        exit 1
        ;;
esac