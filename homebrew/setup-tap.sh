#!/bin/bash
# Script to set up and publish the Homebrew tap for datacell

set -e

echo "Setting up Homebrew tap for datacell..."

# Check if gh CLI is authenticated
if ! gh auth status &>/dev/null; then
    echo "Error: GitHub CLI not authenticated. Run: gh auth login"
    exit 1
fi

# Variables
TAP_REPO="homebrew-datacell"
GITHUB_USER="yingkitw"
FORMULA_FILE="homebrew/Formula/datacell.rb"

# Create tap repository
echo "Creating tap repository: $GITHUB_USER/$TAP_REPO"
gh repo create "$TAP_REPO" \
    --public \
    --description "Homebrew tap for datacell CLI" \
    --clone || echo "Repository may already exist"

# Clone the tap repository
if [ ! -d "$TAP_REPO" ]; then
    git clone "git@github.com:$GITHUB_USER/$TAP_REPO.git"
    cd "$TAP_REPO"
else
    cd "$TAP_REPO"
    git pull origin main
fi

# Set up tap structure
mkdir -p Formula
cp "../$FORMULA_FILE" "Formula/datacell.rb"

# Create README for the tap
cat > README.md << 'EOF'
# homebrew-datacell

Homebrew tap for [datacell](https://github.com/yingkitw/datacell) - A CLI tool for spreadsheet manipulation with pandas-style operations.

## Installation

```bash
brew tap yingkitw/datacell
brew install datacell
```

## Documentation

Full documentation available at: https://github.com/yingkitw/datacell
EOF

# Commit and push
git add Formula/datacell.rb README.md
git commit -m "Add datacell formula

- CLI tool for spreadsheet manipulation
- Supports CSV, Excel, Parquet, Avro formats
- Pandas-style operations" || echo "Nothing to commit"

git push origin main

echo ""
echo "Tap created and published!"
echo ""
echo "To install datacell, run:"
echo "  brew tap $GITHUB_USER/datacell"
echo "  brew install datacell"
echo ""
echo "To uninstall:"
echo "  brew uninstall datacell"
echo "  brew untap $GITHUB_USER/datacell"

cd ..
