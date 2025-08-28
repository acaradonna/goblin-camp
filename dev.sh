#!/bin/bash
# Development helper script for Goblin Camp

set -e

# Enable performance optimizations
export CARGO_INCREMENTAL=1
export CARGO_NET_RETRY=10
export CARGO_NET_TIMEOUT=60

case "$1" in
    "setup"|"")
        echo "Setting up Goblin Camp development environment..."
        echo "Building project with optimizations..."
        
        # Use nextest if available for faster testing
        if command -v cargo-nextest &> /dev/null; then
            echo "Using cargo-nextest for faster test execution..."
            cargo build
            cargo nextest run
        else
            cargo build
            cargo test
        fi
        
        echo "✓ Setup complete! Try: ./dev.sh demo"
        echo "  💡 Tip: Install cargo-nextest for faster testing: cargo install cargo-nextest"
        ;;
    "test")
        echo "Running tests..."
        
        # Use nextest if available, fallback to regular cargo test
        if command -v cargo-nextest &> /dev/null; then
            echo "Using cargo-nextest for faster execution..."
            cargo nextest run
        else
            cargo test
        fi
        ;;
    "test-fast")
        echo "Running fast tests (unit tests only)..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run --lib
        else
            cargo test --lib
        fi
        ;;
    "lint")
        echo "Running clippy linter..."
        cargo clippy --workspace --all-targets --all-features
        ;;
    "lint-fix")
        echo "Running clippy with auto-fixes..."
        cargo clippy --workspace --all-targets --all-features --fix --allow-dirty
        ;;
    "format")
        echo "Formatting code..."
        cargo fmt --all
        ;;
    "check")
        echo "Running full checks (format, lint, test)..."
        cargo fmt --check || (echo "❌ Code needs formatting. Run: ./dev.sh format" && exit 1)
        cargo clippy --workspace --all-targets --all-features
        
        # Use nextest if available
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run
        else
            cargo test
        fi
        
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
    "bench")
        echo "Running performance benchmarks..."
        cargo bench --package gc_core
        echo "✓ Benchmarks completed. Results in target/criterion/"
        ;;
    "bench-baseline")
        echo "Setting new baseline for benchmarks..."
        cargo bench --package gc_core -- --save-baseline baseline
        echo "✓ New baseline saved. Future benchmarks will compare against this."
        ;;
    "audit")
        echo "Running security audit..."
        if ! command -v cargo-audit &> /dev/null; then
            echo "Installing cargo-audit..."
            cargo install cargo-audit --quiet
        fi
        cargo audit
        echo "✓ Security audit completed"
        ;;
    "deny")
        echo "Running license and policy checks..."
        if ! command -v cargo-deny &> /dev/null; then
            echo "Installing cargo-deny..."
            cargo install cargo-deny --quiet
        fi
        
        # Create minimal deny.toml if it doesn't exist
        if [ ! -f "deny.toml" ]; then
            echo "Creating basic deny.toml configuration..."
            cat > deny.toml << 'EOF'
[advisories]
version = 2
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
version = 2
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC"]
deny = ["GPL-2.0", "GPL-3.0", "AGPL-1.0", "AGPL-3.0"]

[bans]
version = 2
multiple-versions = "warn"

[sources]
version = 2
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
EOF
        fi
        
        cargo deny check
        echo "✓ License and policy checks completed"
        ;;
    "nextest-install")
        echo "Installing cargo-nextest for faster testing..."
        cargo install cargo-nextest --quiet
        echo "✓ cargo-nextest installed. Tests will now run faster!"
        ;;
    "tools-install")
        echo "Installing development tools..."
        echo "This may take a few minutes..."
        
        tools=(
            "cargo-nextest"      # Faster test execution
            "cargo-llvm-cov"     # Code coverage
            "cargo-audit"        # Security audit
            "cargo-deny"         # License checking
            "cargo-watch"        # File watching
            "cargo-expand"       # Macro expansion
        )
        
        for tool in "${tools[@]}"; do
            if ! command -v "$tool" &> /dev/null; then
                echo "Installing $tool..."
                cargo install "$tool" --quiet
            else
                echo "$tool already installed"
            fi
        done
        
        echo "✓ All development tools installed!"
        ;;
    "watch")
        echo "Watching for changes and running checks..."
        if ! command -v cargo-watch &> /dev/null; then
            echo "Installing cargo-watch..."
            cargo install cargo-watch --quiet
        fi
        cargo watch -x check -x test
        ;;
    "clean-all")
        echo "Cleaning all build artifacts and caches..."
        cargo clean
        rm -rf target/coverage target/criterion
        echo "✓ All build artifacts cleaned"
        ;;
    "demo")
        echo "Running interactive demo menu..."
        cargo run -p gc_cli -- menu
        ;;
    "build")
        echo "Building project..."
        cargo build
        ;;
    "build-release")
        echo "Building release version..."
        cargo build --release
        echo "✓ Release build completed"
        echo "  Binary: target/release/gc_cli"
        ;;
    "ci-local")
        echo "Running CI checks locally..."
        echo "This simulates the CI pipeline for faster feedback"
        echo ""
        
        echo "🎨 1/4 Format check..."
        cargo fmt --check || (echo "❌ Format check failed" && exit 1)
        
        echo "📋 2/4 Clippy lints..."
        cargo clippy --workspace --all-targets --all-features -- -D warnings || (echo "❌ Clippy failed" && exit 1)
        
        echo "🔨 3/4 Build..."
        cargo build || (echo "❌ Build failed" && exit 1)
        
        echo "🧪 4/4 Tests..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run || (echo "❌ Tests failed" && exit 1)
        else
            cargo test || (echo "❌ Tests failed" && exit 1)
        fi
        
        echo ""
        echo "✅ All CI checks passed locally! 🎉"
        echo "Your changes should pass CI when pushed."
        ;;
    "help")
        echo "Goblin Camp development script"
        echo ""
        echo "Usage: ./dev.sh [command]"
        echo ""
        echo "🔧 Essential Commands:"
        echo "  setup          Setup development environment (default)"
        echo "  build          Build the project"
        echo "  test           Run all tests"
        echo "  test-fast      Run unit tests only (faster)"
        echo "  check          Run format check, lint, and tests (matches CI validation)"
        echo "  ci-local       Run complete CI pipeline locally"
        echo ""
        echo "🎨 Code Quality:"
        echo "  format         Format code with rustfmt"
        echo "  lint           Run clippy linter"
        echo "  lint-fix       Run clippy with auto-fixes"
        echo ""
        echo "📊 Analysis & Reporting:"
        echo "  coverage       Generate code coverage reports (HTML + LCOV)"
        echo "  coverage-check Run coverage with minimum threshold enforcement"
        echo "  bench          Run performance benchmarks"
        echo "  bench-baseline Set new benchmark baseline"
        echo ""
        echo "🔒 Security & Compliance:"
        echo "  audit          Run security vulnerability audit"
        echo "  deny           Run license and policy checks"
        echo ""
        echo "🛠️ Development Tools:"
        echo "  tools-install  Install all development tools"
        echo "  nextest-install Install cargo-nextest for faster testing"
        echo "  watch          Watch for changes and auto-run checks"
        echo "  clean-all      Clean all build artifacts and caches"
        echo ""
        echo "🎮 Demo & Testing:"
        echo "  demo           Run interactive demo menu"
        echo "  build-release  Build optimized release version"
        echo ""
        echo "Examples:"
        echo "  ./dev.sh                # Setup environment"
        echo "  ./dev.sh test           # Run tests"
        echo "  ./dev.sh ci-local       # Full CI validation locally"
        echo "  ./dev.sh check          # Quick validation (same as CI)"
        echo "  ./dev.sh coverage       # Generate coverage reports (core library)"
        echo "  ./dev.sh tools-install  # Install all dev tools for better experience"
        echo "  ./dev.sh demo           # Try the demos"
        echo ""
        echo "💡 Tips:"
        echo "  - Run './dev.sh tools-install' once for the best development experience"
        echo "  - Use './dev.sh ci-local' to test your changes before pushing"
        echo "  - All PRs are automatically validated by CI with enhanced workflows"
        echo "  - Code coverage is measured for core library only (excludes CLI/UI)"
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo "Run './dev.sh help' for available commands"
        exit 1
        ;;
esac