# 📊 Code Coverage Guide

> *Comprehensive guide to code coverage tools and practices for Goblin Camp*

This guide covers code coverage tooling, thresholds, and best practices for maintaining high code quality in Goblin Camp.

## 🎯 Quick Navigation

- [Coverage Tools](#coverage-tools) - Tools and setup for coverage measurement
- [Running Coverage](#running-coverage) - Commands and local usage
- [Coverage Standards](#coverage-standards) - Thresholds and quality gates
- [CI Integration](#ci-integration) - Automated coverage in build pipeline
- [Coverage Reports](#coverage-reports) - Understanding and viewing reports
- [SonarQube Integration](#sonarqube-integration) - External quality analysis

---

## 🛠️ Coverage Tools

### Primary Tool: cargo-llvm-cov

Goblin Camp uses `cargo-llvm-cov` for code coverage measurement:

- **Accuracy**: LLVM-based coverage is more accurate than source-based tools
- **Speed**: Fast execution with minimal overhead
- **Formats**: Supports HTML, LCOV, JSON, and text output formats
- **Integration**: Works seamlessly with SonarQube and CI systems

### Alternative: SonarQube

For comprehensive code quality analysis beyond coverage:

- **Quality Gates**: Enforces coverage thresholds and code quality metrics
- **Historical Tracking**: Track coverage trends over time
- **Code Smells**: Identifies maintainability issues
- **Security**: Detects potential security vulnerabilities

---

## 🚀 Running Coverage

### Local Development Commands

```bash
# Generate HTML coverage report for local viewing
./dev.sh coverage

# Check coverage meets minimum thresholds
./dev.sh coverage-check

# View coverage with exclusions (core library only)
cargo llvm-cov --hide-instantiations --ignore-filename-regex="main\.rs" --summary-only
```

### Manual Coverage Commands

```bash
# Install coverage tool
cargo install cargo-llvm-cov

# Generate HTML report
cargo llvm-cov --html --output-dir target/coverage

# Generate LCOV for external tools
cargo llvm-cov --lcov --output-path target/coverage/lcov.info

# Check specific threshold
cargo llvm-cov --fail-under-lines 75 --summary-only

# Exclude specific files/patterns
cargo llvm-cov --ignore-filename-regex="main\.rs|bench.*\.rs" --summary-only
```

### Coverage Report Locations

```
target/coverage/
├── html/               # HTML coverage report
│   └── index.html     # Open this in browser
├── lcov.info          # LCOV format for SonarQube
└── coverage.json      # JSON format for tools
```

---

## 📈 Coverage Standards

### Quality Thresholds

**Overall Project Coverage**:
- **Minimum**: 65% line coverage (accounts for CLI UI code)
- **Target**: 70% line coverage overall

**Core Library Coverage** (excluding CLI):
- **Minimum**: 85% line coverage
- **Target**: 90% line coverage
- **Current**: 94.58% line coverage ✅

**Function Coverage**:
- **Minimum**: 80% function coverage
- **Current**: 95.52% function coverage ✅

### Per-Module Breakdown

| Module | Coverage | Target | Status |
|--------|----------|---------|--------|
| `lib.rs` | 100% | 100% | ✅ |
| `path.rs` | 100% | 90% | ✅ |
| `stockpiles.rs` | 100% | 90% | ✅ |
| `systems.rs` | 94.26% | 90% | ✅ |
| `world.rs` | 97.22% | 90% | ✅ |
| `save.rs` | 97.65% | 90% | ✅ |
| `fov.rs` | 98.28% | 90% | ✅ |
| `jobs.rs` | 92.31% | 90% | ✅ |
| `inventory.rs` | 90.00% | 90% | ✅ |
| `mapgen.rs` | 87.50% | 85% | ✅ |
| `designations.rs` | 84.62% | 85% | ⚠️ |

### Exclusions

**Files excluded from coverage requirements**:
- `gc_cli/src/main.rs` - CLI interface and UI code
- Test files (`tests/**/*.rs`)
- Benchmark files (`benches/**/*.rs`)

---

## 🔄 CI Integration

### GitHub Actions Workflow

Coverage is automatically checked in CI:

```yaml
- name: Generate coverage report
  run: |
    cargo llvm-cov --lcov --output-path target/coverage/lcov.info
    cargo llvm-cov --html --output-dir target/coverage/html
    cargo llvm-cov --summary-only

- name: Check coverage threshold
  run: |
    cargo llvm-cov --fail-under-lines 65 --summary-only
```

### Coverage Artifacts

CI uploads coverage reports as artifacts:
- HTML reports for manual review
- LCOV files for SonarQube integration
- Coverage summaries in build logs

### Build Failure Conditions

Builds fail if:
- Overall coverage drops below 65%
- Core library coverage drops below 90%
- Any critical path is completely uncovered

---

## 📊 Coverage Reports

### HTML Report Structure

```
target/coverage/html/
├── index.html          # Main coverage dashboard
├── src/               # Source file coverage
│   ├── lib.rs.html
│   ├── systems.rs.html
│   └── ...
└── static/            # CSS and JS assets
```

### Understanding Coverage Metrics

**Line Coverage**: Percentage of executable lines that were run during tests
- **Green**: Lines executed during tests
- **Red**: Lines not executed during tests
- **Gray**: Non-executable lines (comments, blank lines)

**Function Coverage**: Percentage of functions that were called during tests

**Region Coverage**: LLVM-specific metric for code regions

### Reading Coverage Reports

1. **Open HTML Report**: `target/coverage/html/index.html`
2. **Review Summary**: Overall percentages and file breakdown
3. **Drill Down**: Click files to see line-by-line coverage
4. **Identify Gaps**: Red lines indicate uncovered code paths
5. **Add Tests**: Write tests to cover missing paths

---

## 🔍 SonarQube Integration

### Setup Configuration

File: `sonar-project.properties`

```properties
# Project identification
sonar.projectKey=goblin-camp
sonar.projectName=Goblin Camp
sonar.projectVersion=0.1.0

# Source and test directories
sonar.sources=crates/
sonar.tests=crates/gc_core/tests/,crates/gc_cli/tests/
sonar.exclusions=**/target/**,**/*.rs.bk

# Coverage configuration
sonar.coverageReportPaths=target/coverage/lcov.info
sonar.rust.lcov.reportPaths=target/coverage/lcov.info

# Quality gate settings
sonar.qualitygate.wait=true
sonar.coverage.exclusions=**/main.rs,**/tests/**,**/benches/**
```

### Running SonarQube Scanner

```bash
# Generate coverage report
./dev.sh coverage

# Run SonarQube scanner (requires SonarQube server)
sonar-scanner
```

### Quality Gates

SonarQube enforces:
- **Coverage**: ≥ 65% overall, ≥ 90% for core library
- **Maintainability**: A rating on technical debt
- **Reliability**: A rating on bugs and issues
- **Security**: A rating on security vulnerabilities

---

## 🎯 Best Practices

### Writing Coverage-Focused Tests

**Test Critical Paths**:
```rust
#[test]
fn test_error_handling_paths() {
    // Test both success and failure cases
    assert!(operation_with_valid_input().is_ok());
    assert!(operation_with_invalid_input().is_err());
}
```

**Test Edge Cases**:
```rust
#[test]
fn test_boundary_conditions() {
    // Test edge cases that might be missed
    assert_eq!(pathfind_same_position(pos), Some(vec![pos]));
    assert_eq!(pathfind_impossible(), None);
}
```

**Test All Enum Variants**:
```rust
#[test]
fn test_all_job_types() {
    for job_kind in [JobKind::Mine { x: 0, y: 0 }, JobKind::Haul { from: (0,0), to: (1,1) }] {
        assert!(process_job(job_kind).is_ok());
    }
}
```

### Coverage-Driven Development

1. **Write Test First**: Start with failing test for new functionality
2. **Implement Minimally**: Write just enough code to make test pass
3. **Check Coverage**: Ensure new code is covered
4. **Refactor Safely**: Coverage protects against regressions

### Improving Coverage

**Identify Uncovered Code**:
```bash
# Find files with low coverage
cargo llvm-cov --summary-only | grep -E '[0-9]{1,2}\.[0-9]+%'
```

**Focus on Critical Modules**:
- Prioritize systems with complex logic
- Ensure error handling paths are tested
- Test state transitions and edge cases

**Don't Over-Optimize**:
- Aim for meaningful coverage, not just high percentages
- Focus on testing behavior, not implementation details
- Exclude trivial code (getters/setters) when appropriate

---

## 🔧 Troubleshooting

### Common Issues

**Coverage Tool Not Found**:
```bash
error: cargo-llvm-cov not found
```
*Solution*: Install with `cargo install cargo-llvm-cov`

**LLVM Tools Missing**:
```bash
error: llvm-tools-preview component not found
```
*Solution*: Install with `rustup component add llvm-tools-preview`

**Permission Errors**:
```bash
error: cannot write to target/coverage/
```
*Solution*: Ensure write permissions or run `cargo clean`

### Performance Optimization

**Slow Coverage Generation**:
- Use `--workspace` flag to test all crates together
- Consider excluding large test suites from coverage runs
- Use parallel test execution with `--jobs` flag

**Large Coverage Reports**:
- Use `--ignore-filename-regex` to exclude unnecessary files
- Generate summary-only reports for quick checks
- Clean old coverage data regularly

---

## 📚 Related Documentation

- [Testing Guide](./testing.md) - Comprehensive testing strategies
- [Contributing Guide](./contributing.md) - Development workflow and standards
- [CI/CD Documentation](../ci-cd.md) - Build pipeline and automation
- [Quality Assurance](./qa.md) - Overall quality practices

---

*This guide ensures Goblin Camp maintains industry-standard code coverage while providing practical tools for developers to write effective tests and measure code quality.*