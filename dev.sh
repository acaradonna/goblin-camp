#!/bin/bash
# Development helper script for Goblin Camp

set -e

# Enable performance optimizations
export CARGO_INCREMENTAL=1
export CARGO_NET_RETRY=10
export CARGO_NET_TIMEOUT=60

# Configuration
DEMO_TIMEOUT=${DEMO_TIMEOUT:-60s}

# Helper functions
check_clean() {
    for arg in "$@"; do
        if [[ "$arg" == "--clean" ]]; then
            echo "🧹 Cleaning build artifacts (--clean detected)..."
            cargo clean
            break
        fi
    done
}

run_format_check() {
    echo "🎨 Checking format..."
    cargo fmt --all -- --check || (echo "❌ Code needs formatting. Run: ./dev.sh format" && exit 1)
}

run_clippy() {
    echo "🔍 Running clippy..."
    cargo clippy --workspace --all-targets --all-features -- -D warnings || (echo "❌ Clippy failed" && exit 1)
}

run_unit_tests() {
    echo "🧪 Running unit tests..."
    if command -v cargo-nextest &> /dev/null; then
        cargo nextest run --lib || (echo "❌ Unit tests failed" && exit 1)
    else
        cargo test --lib || (echo "❌ Unit tests failed" && exit 1)
    fi
}

run_integration_tests() {
    echo "🧪 Running integration tests..."
    if command -v cargo-nextest &> /dev/null; then
        cargo nextest run --workspace || (echo "❌ Integration tests failed" && exit 1)
    else
        cargo test --workspace || (echo "❌ Integration tests failed" && exit 1)
    fi
}

run_doc_tests() {
    echo "📚 Running doc tests..."
    cargo test --doc --workspace || (echo "❌ Doc tests failed" && exit 1)
}

run_demos() {
    echo "🎮 Validating demos (timeout: $DEMO_TIMEOUT)..."

    echo "  Testing map generation..."
    timeout "$DEMO_TIMEOUT" cargo run -p gc_cli -- --width 20 --height 10 mapgen > /dev/null || (echo "❌ Map generation demo failed" && exit 1)

    echo "  Testing pathfinding..."
    timeout "$DEMO_TIMEOUT" cargo run -p gc_cli -- --width 30 --height 15 path > /dev/null || (echo "❌ Pathfinding demo failed" && exit 1)

    echo "  Testing save/load..."
    timeout "$DEMO_TIMEOUT" cargo run -p gc_cli -- save-load > /dev/null || (echo "❌ Save/load demo failed" && exit 1)

    echo "  Testing field of view..."
    timeout "$DEMO_TIMEOUT" cargo run -p gc_cli -- fov > /dev/null || (echo "❌ FOV demo failed" && exit 1)

    echo "✅ All demos working"
}

run_pr_validate() {
    echo "🔍 Validating PR format..."
    if [[ -f "./scripts/validate-pr.sh" ]]; then
        ./scripts/validate-pr.sh || (echo "❌ PR validation failed" && exit 1)
    else
        echo "⚠️  PR validation script not found, skipping"
    fi
}

run_pre_push_validation() {
    echo "🔍 Running pre-push validation (branch + commits)..."

    # Ensure we have an up-to-date main for commit range checks
    git fetch origin main --quiet || true

    local branch
    branch="$(git rev-parse --abbrev-ref HEAD)"
    local base_range="origin/main..HEAD"

    if [[ -f "./scripts/validate-pr.sh" ]]; then
        ./scripts/validate-pr.sh --branch-name "$branch" --commit-range "$base_range" || (echo "❌ Pre-push PR validation failed" && exit 1)
    else
        echo "⚠️  PR validation script not found, skipping branch/commit checks"
    fi
}

# Main command dispatcher
case "$1" in
    "setup"|"")
        echo "Setting up Goblin Camp development environment..."
        check_clean "$@"
        echo "Building project..."
        cargo build

        run_unit_tests

        echo "✓ Setup complete! Try: ./dev.sh demo"
        echo "  💡 Tip: Install cargo-nextest for faster testing: cargo install cargo-nextest"
        ;;

    "fast")
        # Target <30s - Inner Loop
        echo "⚡ Running FAST validation..."
        check_clean "$@"

        run_format_check

        echo "🔍 Cargo Check..."
        cargo check --workspace || (echo "❌ Check failed" && exit 1)

        run_unit_tests

        echo "✅ Fast validation complete!"
        ;;

    "agent")
        # Target ~1m - Agent/Pre-PR
        echo "🤖 Running AGENT validation..."
        check_clean "$@"

        run_pr_validate
        run_format_check
        run_clippy

        echo "🧪 Running all tests..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run --workspace || (echo "❌ Tests failed" && exit 1)
        else
            cargo test --workspace || (echo "❌ Tests failed" && exit 1)
        fi

        echo "✅ Agent validation complete! Ready for review."
        ;;

    "pre-push")
        # Pre-push safety net: branch/commit validation + agent checks
        echo "🚦 Running PRE-PUSH validation..."
        check_clean "$@"

        run_pre_push_validation
        run_format_check
        run_clippy

        echo "🧪 Running all tests..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run --workspace || (echo "❌ Tests failed" && exit 1)
        else
            cargo test --workspace || (echo "❌ Tests failed" && exit 1)
        fi

        echo "✅ Pre-push validation complete! Safe to push."
        ;;

    "full"|"ci-simulate")
        # Target ~5m - Golden Path / CI Simulation
        echo "🚀 Running FULL validation (CI Simulation)..."
        check_clean "$@"

        run_pr_validate
        run_format_check
        run_clippy

        echo "🔨 Building (Debug & Release)..."
        cargo build --verbose
        cargo build --release --verbose

        echo "🧪 Running all tests (including docs)..."
        if command -v cargo-nextest &> /dev/null; then
            cargo nextest run --workspace
        else
            cargo test --workspace
        fi
        run_doc_tests

        run_demos

        # Run coverage check if available
        echo "📊 Checking coverage..."
        if command -v cargo-llvm-cov &> /dev/null; then
            cargo llvm-cov --fail-under-lines 75 --summary-only --package gc_core || echo "⚠️ Coverage below threshold (non-fatal for local full run)"
        else
            echo "⚠️ cargo-llvm-cov not found, skipping coverage check"
        fi

        echo "🎉 Full validation complete!"
        ;;

    "sync")
        echo "🔄 Syncing with origin/main..."
        git fetch origin
        if git rebase origin/main; then
            echo "📦 Updating dependencies..."
            cargo update --workspace
            echo "✅ Sync complete!"
        else
            echo "❌ Rebase failed. Please resolve conflicts manually."
            exit 1
        fi
        ;;

    # Legacy/Specific commands preserved for compatibility
    "test")
        run_unit_tests
        run_integration_tests
        ;;
    "test-fast")
        run_unit_tests
        ;;
    "lint")
        cargo clippy --workspace --all-targets --all-features
        ;;
    "lint-fix")
        cargo clippy --workspace --all-targets --all-features --fix --allow-dirty
        ;;
    "format")
        cargo fmt --all
        ;;
    "pr-validate")
        run_pr_validate
        ;;
    "check"|"validate"|"ci-local")
        # Alias to agent for roughly equivalent behavior, but warn
        echo "ℹ️  Legacy command '$1' detected. Running 'agent' validation..."
        "$0" agent "$@"
        ;;
    "demo")
        echo "Running interactive demo menu..."
        cargo run -p gc_cli -- menu
        ;;
    "coverage")
        echo "Generating code coverage report..."
        cargo install cargo-llvm-cov --quiet || true
        cargo llvm-cov --html --output-dir target/coverage --package gc_core
        cargo llvm-cov --lcov --output-path target/coverage/lcov.info --package gc_core
        echo "✓ Coverage reports generated in target/coverage/"
        ;;
    "coverage-check")
        if ! command -v cargo-llvm-cov &> /dev/null; then
            cargo install cargo-llvm-cov --quiet
        fi
        cargo llvm-cov --fail-under-lines 75 --summary-only --package gc_core
        ;;
    "tools-install")
        echo "Installing development tools..."
        tools=("cargo-nextest" "cargo-llvm-cov" "cargo-audit" "cargo-deny" "cargo-watch" "cargo-expand")
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
    "clean-all")
        echo "Cleaning all build artifacts..."
        cargo clean
        rm -rf target/coverage target/criterion
        echo "✓ Cleaned"
        ;;
    "help"|*)
        echo "Goblin Camp development script"
        echo "Usage: ./dev.sh [command] [--clean]"
        echo ""
        echo "🚀 Core Workflows:"
        echo "  fast           <30s  Format, Check, Unit Tests (Inner Loop)"
        echo "  agent          ~1m   Fast + Clippy + All Tests (Agent/Pre-Review)"
        echo "  full           ~5m   Agent + Release Build + Docs + Demos (CI Simulation)"
        echo "  sync                 Fetch, Rebase origin/main, Update deps"
        echo ""
        echo "🛠️  Specific Tasks:"
        echo "  setup          Setup environment"
        echo "  demo           Run interactive demo menu"
        echo "  format         Format code"
        echo "  lint-fix       Fix clippy issues"
        echo "  coverage       Generate coverage report"
        echo "  tools-install  Install dev tools"
        echo ""
        echo "Flags:"
        echo "  --clean        Run 'cargo clean' before command"
        ;;
esac
