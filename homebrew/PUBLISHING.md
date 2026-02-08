# Publishing datacell to Homebrew

This guide will walk you through publishing datacell to Homebrew.

## Prerequisites

1. GitHub account with access to yingkitw/datacell
2. Homebrew installed on your Mac/Linux machine
3. Git installed

## Step 1: Re-authenticate GitHub CLI (if needed)

```bash
gh auth login
```

Follow the prompts to authenticate.

## Step 2: Create the Homebrew Tap Repository

### Option A: Using the Setup Script (Recommended)

```bash
cd /path/to/datacell
./homebrew/setup-tap.sh
```

This script will:
- Create the homebrew-datacell repository on GitHub
- Set up the proper tap structure
- Push the formula to the repository

### Option B: Manual Setup

1. **Create the tap repository on GitHub:**

```bash
gh repo create homebrew-datacell \
    --public \
    --description "Homebrew tap for datacell CLI"
```

2. **Clone and set up the tap:**

```bash
# Clone the tap repository
git clone git@github.com:yingkitw/homebrew-datacell.git
cd homebrew-datacell

# Create Formula directory
mkdir -p Formula

# Copy the formula from datacell repository
cp ../datacell/homebrew/Formula/datacell.rb Formula/

# Copy README
cp ../datacell/homebrew/README.md README.md

# Commit and push
git add Formula/datacell.rb README.md
git commit -m "Add datacell formula

- CLI tool for spreadsheet manipulation
- Supports CSV, Excel, Parquet, Avro formats
- Pandas-style operations"

git push origin main

cd ..
```

## Step 3: Install from the Tap

Once the tap is created, anyone can install datacell:

```bash
# Add the tap
brew tap yingkitw/datacell

# Install datacell
brew install datacell

# Verify installation
datacell --version
```

## Step 4: Test the Installation

```bash
# Test basic functionality
datacell read --input examples/data.csv

# Test conversion
datacell convert --input examples/data.csv --output test.xlsx

# Test formula
datacell formula --input examples/data.csv --output result.csv \
    --formula "SUM(A1:A10)" --cell "C1"

# Clean up test files
rm test.xlsx result.csv
```

## Step 5: Create a GitHub Release (Optional but Recommended)

To support stable releases with versioned downloads:

1. **Tag the release:**

```bash
cd /path/to/datacell

# Create and push tag
git tag v0.1.5
git push origin v0.1.5
```

2. **Create a GitHub release:**

```bash
gh release create v0.1.5 \
    --title "v0.1.5 - Performance Improvements" \
    --notes "## Performance Improvements

- Added compiler optimizations (LTO, opt-level 3)
- Cached regex compilation for better performance
- Added rayon parallel processing for data operations
- Eliminated duplicate code in converter
- Fixed CSV parsing to use proper csv crate
- Added memory pre-allocation hints

## Installation

\`\`\`bash
brew tap yingkitw/datacell
brew install datacell
\`\`\`

## Full Changelog

See [CHANGELOG.md](https://github.com/yingkitw/datacell/blob/main/CHANGELOG.md)"
```

3. **Build and attach release binaries:**

```bash
# Build for current platform
cargo build --release

# Create release archive
tar -czf datacell-v0.1.5-aarch64-apple-darwin.tar.gz \
    -C target/release datacell

# Upload to release
gh release upload v0.1.5 \
    datacell-v0.1.5-aarch64-apple-darwin.tar.gz
```

## Step 6: Update the Formula for Releases (Optional)

If you created a GitHub release, update `Formula/datacell.rb` to use release archives:

```ruby
class Datacell < Formula
  desc "CLI tool for spreadsheet manipulation with pandas-style operations"
  homepage "https://github.com/yingkitw/datacell"

  url "https://github.com/yingkitw/datacell/archive/refs/tags/v0.1.5.tar.gz"
  sha256 ""  # Run: shasum -a 256 v0.1.5.tar.gz

  # ... rest of the formula
end
```

To get the SHA256 checksum:
```bash
curl -L https://github.com/yingkitw/datacell/archive/refs/tags/v0.1.5.tar.gz | shasum -a 256
```

## Step 7: Publish to crates.io (Optional)

To make datacell available via `cargo install`:

```bash
# Login to crates.io
cargo login

# Publish to crates.io
cargo publish
```

## Step 8: Submit to Homebrew Core (Optional - For Official Inclusion)

To have datacell included in the official Homebrew repository:

1. Fork https://github.com/Homebrew/homebrew-core
2. Copy your formula to the fork:
   ```bash
   git clone git@github.com:yingkitw/homebrew-core.git
   cd homebrew-core
   cp ../datacell/homebrew/Formula/datacell.rb Formula/datacell.rb
   ```
3. Test the formula:
   ```bash
   brew install --build-from-source ./Formula/datacell.rb
   brew test datacell
   ```
4. Submit a PR following the guidelines at:
   https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md

## Maintenance

### Updating for New Releases

1. Update version in `Cargo.toml`
2. Create a new git tag
3. Create a GitHub release
4. Update the formula in homebrew-datacell tap
5. Bump the version in the tap

### Updating the Tap

```bash
cd homebrew-datacell

# Edit Formula/datacell.rb with new version
# Update the URL and sha256

# Test the update
brew uninstall datacell
brew install --build-from-source ./Formula/datacell.rb

# Commit and push
git add Formula/datacell.rb
git commit -m "Update datacell to v0.1.6"
git push origin main
```

## Users Can Update

```bash
brew upgrade datacell
```

## Troubleshooting

### Formula Not Found

```bash
# Make sure the tap is added
brew tap-info yingkitw/datacell

# Re-add the tap
brew untap yingkitw/datacell
brew tap yingkitw/datacell
```

### Build Failures

```bash
# Update Homebrew
brew update

# Upgrade Rust
brew upgrade rust

# Reinstall from source
brew reinstall datacell --build-from-source
```

### Permission Errors

```bash
# Fix Homebrew permissions
sudo chown -R $(whoami) /usr/local/lib/node_modules
brew doctor
```

## Success Metrics

After publishing, you can track:

```bash
# See installations (if you have analytics enabled)
brew info datacell

# Check for issues
brew audit datacell
brew style datacell
```

## Resources

- Homebrew Formula Cookbook: https://docs.brew.sh/Formula-Cookbook
- Homebrew Ruby API: https://rubydoc.brew.sh/Formula
- Homebrew Contributing Guidelines: https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md

## Quick Reference

```bash
# User commands
brew tap yingkitw/datacell          # Add the tap
brew install datacell                # Install
brew upgrade datacell                # Upgrade
brew uninstall datacell              # Remove
brew untap yingkitw/datacell         # Remove tap

# Maintainer commands
brew audit --new-formula Formula/datacell.rb   # Check formula
brew style Formula/datacell.rb                # Check style
brew test --build-from-source Formula/datacell.rb  # Test formula

# Release commands
git tag v0.1.6 && git push --tags                 # Create tag
gh release create v0.1.6 --notes "Release notes"  # Create release
cargo publish                                      # Publish to crates.io
```

## Next Steps

1. ✅ Formula created in `homebrew/Formula/datacell.rb`
2. ✅ Setup script created in `homebrew/setup-tap.sh`
3. ✅ README updated with Homebrew instructions
4. ⏳ Run `./homebrew/setup-tap.sh` to create the tap
5. ⏳ Test installation with `brew tap yingkitw/datacell && brew install datacell`
6. ⏳ Create GitHub release for v0.1.5
7. ⏳ Consider publishing to crates.io
