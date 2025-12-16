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
    "pr-validate")
        echo "Validating PR commit messages and branch name..."
        if [[ -f "./scripts/validate-pr.sh" ]]; then
            ./scripts/validate-pr.sh
        else
            echo "❌ PR validation script not found at ./scripts/validate-pr.sh"
            exit 1
        fi
        ;;
    "check")
        echo "Running full validation (matches CI pipeline)..."
        
        echo "🔍 Validating PR format..."
        if [[ -f "./scripts/validate-pr.sh" ]]; then
            ./scripts/validate-pr.sh || (echo "❌ PR validation failed" && exit 1)
        else
            echo "⚠️  PR validation script not found, skipping"
        fi

        echo "🎨 Checking format..."
        cargo fmt --all -- --check || (echo "❌ Code needs formatting. Run: ./dev.sh format" && exit 1)

        echo "🔍 Running clippy..."
        cargo clippy --workspace --all-targets --all-features

        echo "🧪 Running tests..."
        # Use nextest if available for faster testing
        if command -v cargo-nextest &> /dev/null; then
            echo "Using cargo-nextest for faster execution..."
            cargo nextest run --workspace
        else
            cargo test --workspace
        fi

        echo "📚 Running doc tests..."
        cargo test --doc --workspace

        echo "✅ Local validation complete!"
        echo "💡 Tip: Run './dev.sh coverage-check' for coverage validation"
        ;;
    "validate")
        echo "Running comprehensive validation (includes coverage & demos)..."
        
        echo "Step 1/6: PR validation..."
        if [[ -f "./scripts/validate-pr.sh" ]]; then
            ./scripts/validate-pr.sh || (echo "❌ PR validation failed" && exit 1)
        else
            echo "⚠️  PR validation script not found, skipping"
        fi
        
        echo "Step 2/6: Format check..."
        cargo fmt --all -- --check || (echo "❌ Code needs formatting. Run: ./dev.sh format" && exit 1)

        echo "Step 3/6: Clippy lint..."
        cargo clippy --workspace --all-targets --all-features

        echo "Step 4/6: Build check..."
        cargo build --verbose
        cargo build --release --verbose

        echo "Step 5/6: Test suite..."
        # Use nextest if available for faster testing
        if command -v cargo-nextest &> /dev/null; then
            echo "Using cargo-nextest for faster execution..."
            cargo nextest run --workspace
        else
            cargo test --workspace
        fi
        cargo test --doc --workspace

        echo "Step 6/6: Demo validation..."
        echo "Testing map generation..."
        timeout 30s cargo run -p gc_cli -- --width 20 --height 10 mapgen > /dev/null
        echo "Testing pathfinding..."
        timeout 30s cargo run -p gc_cli -- --width 30 --height 15 path > /dev/null
        echo "Testing save/load..."
        timeout 30s cargo run -p gc_cli -- save-load > /dev/null
        echo "Testing field of view..."
        timeout 30s cargo run -p gc_cli -- fov > /dev/null

        echo "✅ Comprehensive validation complete!"
        echo "💡 Tip: Run './dev.sh coverage-check' to validate coverage meets CI threshold"
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
        echo "Running coverage threshold check (core library)..."
        if ! command -v cargo-llvm-cov &> /dev/null; then
            echo "Installing cargo-llvm-cov..."
            cargo install cargo-llvm-cov --quiet
        fi
        cargo llvm-cov --fail-under-lines 75 --summary-only --package gc_core
        ;;
    "ci-simulate")
        echo "Simulating full CI pipeline locally..."
        echo "🚀 Running CI simulation (this may take a few minutes)..."

        # Step 0: PR Validation
        echo "Step 1/6: 🔍 PR Validation..."
        if [[ -f "./scripts/validate-pr.sh" ]]; then
            ./scripts/validate-pr.sh || (echo "❌ PR validation failed" && exit 1)
        else
            echo "⚠️  PR validation script not found, skipping"
        fi

        # Step 1: Validation
        echo "Step 2/6: 🎨 Format & Lint Validation..."
        cargo fmt --all -- --check || (echo "❌ Format check failed. Run: ./dev.sh format" && exit 1)
        cargo clippy --workspace --all-targets --all-features

        # Step 2: Build
        echo "Step 3/6: 🔨 Build (debug & release)..."
        cargo build --verbose
        cargo build --release --verbose

        # Step 3: Tests
        echo "Step 4/6: 🧪 Test suite..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run --workspace
        else
            cargo test --workspace
        fi
        cargo test --doc --workspace

        # Step 4: Coverage
        echo "Step 5/6: 📊 Coverage check..."
        cargo install cargo-llvm-cov --quiet || true
        cargo llvm-cov --fail-under-lines 75 --summary-only --package gc_core

        # Step 5: Demo validation
        echo "Step 6/6: 🎮 Demo validation..."
        echo "Testing map generation..."
        timeout 30s cargo run -p gc_cli -- --width 20 --height 10 mapgen > /dev/null
        echo "Testing pathfinding..."
        timeout 30s cargo run -p gc_cli -- --width 30 --height 15 path > /dev/null
        echo "Testing save/load..."
        timeout 30s cargo run -p gc_cli -- save-load > /dev/null
        echo "Testing field of view..."
        timeout 30s cargo run -p gc_cli -- fov > /dev/null

        echo "🎉 CI simulation complete! All checks passed!"
        echo "💡 This matches exactly what CI will run"
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

        # Verify workspace consistency first
        echo "🔍 Verifying workspace consistency..."
        for member in $(grep -A 10 '^\[workspace\]' Cargo.toml | grep -E '^\s*"' | tr -d '",' | xargs); do
            if [ ! -d "$member" ]; then
                echo "❌ Workspace member '$member' does not exist"
                exit 1
            fi
            echo "✅ Found workspace member: $member"
        done

        if ! command -v cargo-audit &> /dev/null; then
            echo "Installing cargo-audit..."
            cargo install cargo-audit --quiet
        fi
        # Match CI behavior exactly with --deny warnings and ignore unmaintained paste
        cargo audit --deny warnings --ignore RUSTSEC-2024-0436 --color always
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
    "ci-essential")
        echo "Running essential CI checks locally..."
        echo "This runs the core checks that must pass (faster than full ci-local)"
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
        echo "✅ Essential CI checks passed locally! 🎉"
        echo "Run './dev.sh ci-local' for comprehensive validation before pushing."
        ;;
    "ci-local")
        echo "Running comprehensive CI checks locally..."
        echo "This simulates the complete CI pipeline for faster feedback"
        echo ""

        # Step 1: Core CI checks
        echo "🔧 CORE CI CHECKS"
        echo "=================="
        echo ""

        # Step 0: PR Validation (new)
        echo "🔍 0/10 PR validation..."
        if [[ -f "./scripts/validate-pr.sh" ]]; then
            ./scripts/validate-pr.sh || (echo "❌ PR validation failed" && exit 1)
        else
            echo "⚠️  PR validation script not found, skipping"
        fi

        echo "🎨 1/10 Format check..."
        cargo fmt --check || (echo "❌ Format check failed" && exit 1)

        echo "📋 2/10 Clippy lints..."
        cargo clippy --workspace --all-targets --all-features -- -D warnings || (echo "❌ Clippy failed" && exit 1)

        echo "🔨 3/10 Build..."
        cargo build || (echo "❌ Build failed" && exit 1)

        echo "🧪 4/10 Tests..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run || (echo "❌ Tests failed" && exit 1)
        else
            cargo test || (echo "❌ Tests failed" && exit 1)
        fi

        echo "🎮 5/10 Demo validation..."
        echo "  Testing map generation..."
        timeout 30s cargo run -p gc_cli -- --width 20 --height 10 mapgen > /dev/null || (echo "❌ Map generation demo failed" && exit 1)
        echo "  Testing save/load..."
        timeout 30s cargo run -p gc_cli -- save-load > /dev/null || (echo "❌ Save/load demo failed" && exit 1)
        echo "  Testing pathfinding..."
        timeout 30s cargo run -p gc_cli -- --width 30 --height 15 path > /dev/null || (echo "❌ Pathfinding demo failed" && exit 1)
        echo "  Testing field of view..."
        timeout 30s cargo run -p gc_cli -- fov > /dev/null || (echo "❌ FOV demo failed" && exit 1)
        echo "  ✅ All demos working"

        # Step 2: Quality checks
        echo ""
        echo "📊 QUALITY CHECKS"
        echo "=================="
        echo ""

        echo "📊 6/10 Coverage threshold check..."
        if ! command -v cargo-llvm-cov &> /dev/null; then
            echo "  Installing cargo-llvm-cov..."
            cargo install cargo-llvm-cov --quiet
        fi
        cargo llvm-cov --fail-under-lines 75 --summary-only --package gc_core || (echo "❌ Coverage below 75% threshold" && exit 1)
        echo "  ✅ Coverage meets minimum threshold"

        echo "📚 7/10 Documentation check..."
        cargo doc --workspace --no-deps --quiet || (echo "❌ Documentation build failed" && exit 1)
        echo "  ✅ Documentation builds successfully"

        # Step 3: Security checks
        echo ""
        echo "🔒 SECURITY CHECKS"
        echo "=================="
        echo ""

        echo "🔍 8/10 Security audit..."
        if ! command -v cargo-audit &> /dev/null; then
            echo "  Installing cargo-audit..."
            cargo install cargo-audit --quiet
        fi
        # Match CI behavior exactly with --deny warnings and ignore unmaintained paste
        cargo audit --deny warnings --ignore RUSTSEC-2024-0436 --color always || (echo "❌ Security vulnerabilities found" && exit 1)
        echo "  ✅ No security vulnerabilities"

        echo "🚫 9/10 License compliance..."
        if ! command -v cargo-deny &> /dev/null; then
            echo "  Installing cargo-deny..."
            cargo install cargo-deny --quiet
        fi

        cargo deny check --hide-inclusion-graph || (echo "❌ License/policy violations found" && exit 1)
        echo "  ✅ License compliance verified"

        echo ""
        echo "🎉 ALL CI CHECKS PASSED LOCALLY! 🎉"
        echo "=================================="
        echo ""
        echo "✅ PR Validation: Commit messages and branch naming"
        echo "✅ Core CI: Format, lint, build, test, demos"
        echo "✅ Quality: Coverage (≥75%), documentation"
        echo "✅ Security: Vulnerability audit, license compliance"
        echo ""
        echo "Your changes are ready for CI and should pass all checks!"
        echo ""
        echo "💡 Next steps:"
        echo "  - Push your changes to trigger CI"
        echo "  - All workflows should pass based on local validation"
        echo "  - The PR can be moved out of draft once CI is green"
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
        echo "  pr-validate    Validate PR commit messages and branch name"
        echo "  check          Run format check, lint, and tests (matches CI validation)"
        echo "  validate       Run essential CI checks locally (format, lint, build, test)"
        echo "  ci-simulate    Run complete CI pipeline locally (comprehensive validation)"
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
        echo "  ./dev.sh validate       # Quick essential CI validation"
        echo "  ./dev.sh ci-simulate    # Full CI validation locally (comprehensive)"
        echo "  ./dev.sh check          # Legacy validation (same as validate)"
        echo "  ./dev.sh coverage       # Generate coverage reports (core library)"
        echo "  ./dev.sh tools-install  # Install all dev tools for better experience"
        echo "  ./dev.sh demo           # Try the demos"
        echo ""
        echo "💡 Tips:"
        echo "  - Run './dev.sh tools-install' once for the best development experience"
        echo "  - Use './dev.sh validate' for quick feedback during development"
        echo "  - Use './dev.sh ci-simulate' for comprehensive validation before pushing"
        echo "  - All PRs are automatically validated by CI with enhanced workflows"
        echo "  - Code coverage is measured for core library only (excludes CLI/UI)"
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo "Run './dev.sh help' for available commands"
        exit 1
        ;;
esac
