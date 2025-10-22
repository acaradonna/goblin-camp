#!/usr/bin/env bash
set -euo pipefail

# PR Validation Script for Goblin Camp
# Validates commit messages and branch names according to project standards

usage() {
    echo "Usage: $0 [--commit-range RANGE] [--branch-name NAME] [--help]"
    echo ""
    echo "Validates PR commit messages and branch names against project standards."
    echo ""
    echo "Options:"
    echo "  --commit-range RANGE    Validate commits in range (default: origin/main..HEAD)"
    echo "  --branch-name NAME      Validate specific branch name (default: current branch)"
    echo "  --help                  Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Validate current PR"
    echo "  $0 --commit-range HEAD~3..HEAD       # Validate last 3 commits"
    echo "  $0 --branch-name feat/new-feature    # Validate specific branch name"
}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
COMMIT_RANGE=""
BRANCH_NAME=$(git branch --show-current 2>/dev/null || echo "")
ERRORS=0

# Set default commit range based on what's available
if git rev-parse --verify origin/main >/dev/null 2>&1; then
    COMMIT_RANGE="origin/main..HEAD"
elif git rev-parse --verify main >/dev/null 2>&1; then
    COMMIT_RANGE="main..HEAD"
else
    # Fallback: validate the current commit only if no main branch reference
    COMMIT_RANGE="HEAD^!"
fi

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --commit-range)
            COMMIT_RANGE="$2"
            shift 2
            ;;
        --branch-name)
            BRANCH_NAME="$2"
            shift 2
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option '$1'${NC}" >&2
            usage
            exit 1
            ;;
    esac
done

echo -e "${BLUE}🔍 PR Validation for Goblin Camp${NC}"
echo "======================================="
echo ""

# Validate branch name
validate_branch_name() {
    local branch_name="$1"

    echo -e "${BLUE}📋 Validating branch name: ${YELLOW}$branch_name${NC}"

    if [[ -z "$branch_name" ]]; then
        # On PR merges, Actions checks out a detached merge ref; prefer head ref
        if [[ -n "${GITHUB_HEAD_REF:-}" ]]; then
            branch_name="$GITHUB_HEAD_REF"
            echo -e "${BLUE}ℹ️ Using GITHUB_HEAD_REF: ${YELLOW}$branch_name${NC}"
        else
            echo -e "${YELLOW}⚠️ Branch name unknown in detached HEAD; skipping branch name validation${NC}"
            return 0
        fi
    fi

    # Expected patterns: feat/description, feature/description, docs/description,
    # refactor/description, test/description, chore/description
    if [[ "$branch_name" =~ ^(feat|feature|fix|docs|refactor|test|chore)/[a-z0-9-]+ ]]; then
        echo -e "${GREEN}✅ Branch name follows standard pattern${NC}"
        return 0
    else
        echo -e "${RED}❌ Branch name does not follow expected pattern${NC}"
        echo ""
        echo "Expected patterns:"
        echo "  - feat/description-here     (new features)"
        echo "  - feature/description-here  (new features)"
        echo "  - fix/description-here      (bug fixes)"
        echo "  - docs/description-here     (documentation changes)"
        echo "  - refactor/description-here (code refactoring)"
        echo "  - test/description-here     (test additions/changes)"
        echo "  - chore/description-here    (maintenance tasks)"
        echo ""
        echo "Use lowercase letters, numbers, and hyphens only."
        return 1
    fi
}

# Validate individual commit message
validate_commit_message() {
    local commit_sha="$1"
    local commit_subject=$(git log --format="%s" -n 1 "$commit_sha")
    local commit_body=$(git log --format="%b" -n 1 "$commit_sha")
    local errors=0

    echo -e "${BLUE}📝 Validating commit: ${YELLOW}${commit_sha:0:8}${NC}"
    echo "Subject: $commit_subject"

    # Check subject line length (should be ≤ 50 characters)
    if [[ ${#commit_subject} -gt 50 ]]; then
        echo -e "${RED}❌ Subject line too long (${#commit_subject} chars, max 50)${NC}"
        errors=$((errors + 1))
    else
        echo -e "${GREEN}✅ Subject line length OK (${#commit_subject} chars)${NC}"
    fi

    # Check subject line format (should start with lowercase letter or word in parens)
    if [[ "$commit_subject" =~ ^(\([a-z-]+\): |[a-z]) ]]; then
        echo -e "${GREEN}✅ Subject line format OK${NC}"
    else
        echo -e "${RED}❌ Subject line should start with lowercase letter or (scope):${NC}"
        echo "  Examples: 'fix memory leak' or '(core): add new system'"
        errors=$((errors + 1))
    fi

    # Check if subject line ends with period (it shouldn't)
    if [[ "$commit_subject" =~ \.$ ]]; then
        echo -e "${RED}❌ Subject line should not end with period${NC}"
        errors=$((errors + 1))
    else
        echo -e "${GREEN}✅ Subject line ending OK${NC}"
    fi

    # Check body line lengths if body exists
    if [[ -n "$commit_body" ]]; then
        local line_errors=0
        while IFS= read -r line; do
            # Skip empty lines and lines that are just URLs
            if [[ -n "$line" && ! "$line" =~ ^https?:// && ${#line} -gt 72 ]]; then
                if [[ $line_errors -eq 0 ]]; then
                    echo -e "${RED}❌ Body lines exceed 72 characters:${NC}"
                fi
                echo "  Line (${#line} chars): ${line:0:60}..."
                line_errors=$((line_errors + 1))
            fi
        done <<< "$commit_body"

        if [[ $line_errors -eq 0 ]]; then
            echo -e "${GREEN}✅ Body line lengths OK${NC}"
        else
            errors=$((errors + line_errors))
        fi
    fi

    # Check for issue references (recommended but not required)
    if [[ "$commit_subject $commit_body" =~ (Fixes|Closes|Resolves)\ #[0-9]+ ]]; then
        echo -e "${GREEN}✅ Issue reference found${NC}"
    else
        echo -e "${YELLOW}⚠️  No issue reference found (recommended)${NC}"
        echo "  Add 'Fixes #123' or similar to link commits to issues"
    fi

    echo ""
    return $errors
}

# Validate all commits in range
validate_commits() {
    local commit_range="$1"

    echo -e "${BLUE}📋 Validating commits in range: ${YELLOW}$commit_range${NC}"
    echo ""

    # Get list of commit SHAs in the range (exclude merge commits)
    local commits
    commits=$(git rev-list --no-merges "$commit_range" 2>/dev/null)
    local git_exit_code=$?

    if [[ $git_exit_code -ne 0 ]]; then
        echo -e "${RED}❌ Error: Invalid commit range '$commit_range'${NC}"
        echo "Make sure you have the latest changes from origin/main"
        return 1
    fi

    if [[ -z "$commits" ]]; then
        echo -e "${YELLOW}⚠️  No commits found in range '$commit_range'${NC}"
        return 0
    fi

    local commit_count=$(echo "$commits" | wc -l)
    echo "Found $commit_count commit(s) to validate"
    echo ""

    local total_errors=0
    local commit_num=1

    # Validate each commit (reverse order to show oldest first)
    while IFS= read -r commit_sha; do
        echo "[$commit_num/$commit_count]"
        validate_commit_message "$commit_sha"
        local commit_errors=$?
        total_errors=$((total_errors + commit_errors))
        commit_num=$((commit_num + 1))
    done <<< "$(echo "$commits" | tac)"

    if [[ $total_errors -gt 0 ]]; then
        return $total_errors
    else
        return 0
    fi
}

# Main validation
main() {
    # Validate branch name
    if ! validate_branch_name "$BRANCH_NAME"; then
        ERRORS=$((ERRORS + 1))
    fi

    echo ""

    # Validate commits
    validate_commits "$COMMIT_RANGE"
    local commit_validation_result=$?
    if [[ $commit_validation_result -gt 0 ]]; then
        ERRORS=$((ERRORS + commit_validation_result))
    fi

    # Summary
    echo "======================================="
    if [[ $ERRORS -eq 0 ]]; then
        echo -e "${GREEN}🎉 All validations passed!${NC}"
        echo ""
        echo "Your PR follows the project's commit and branch naming standards."
    else
        echo -e "${RED}❌ Validation failed with $ERRORS error(s)${NC}"
        echo ""
        echo "Please fix the issues above before submitting your PR."
        echo "See docs/developer/contributing.md for detailed guidelines."
        echo ""
        exit 1
    fi
}

main "$@"
