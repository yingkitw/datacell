# Homebrew Installation for datacell

This directory contains the Homebrew formula for installing datacell.

## Installation Options

### Option 1: Install from Custom Tap (Recommended)

Create a tap repository and install from there:

```bash
# 1. Create a new GitHub repository called homebrew-datacell
#    (You can do this via GitHub web interface or gh CLI)

# 2. Clone the tap repository
git clone git@github.com:yingkitw/homebrew-datacell.git
cd homebrew-datacell

# 3. Copy the formula to the tap
cp ../homebrew/datacell.rb Formula/datacell.rb

# 4. Commit and push
git add Formula/datacell.rb
git commit -m "Add datacell formula"
git push origin main

# 5. Install datacell
brew tap yingkitw/datacell
brew install datacell
```

### Option 2: Install Directly from Formula (Local Development)

```bash
# Navigate to the datacell repository
cd /path/to/datacell

# Install from local formula
brew install --formula ./homebrew/datacell.rb
```

### Option 3: Submit to Homebrew Core (For Official Inclusion)

To have datacell included in the official Homebrew repository:

1. Fork https://github.com/Homebrew/homebrew-core
2. Copy `homebrew/datacell.rb` to `Formula/`
3. Follow the guidelines at https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md
4. Submit a Pull Request

## Updating the Formula

When releasing a new version:

1. Update the version in `Cargo.toml`
2. Create a git tag: `git tag v0.1.6 && git push --tags`
3. Create a GitHub release
4. Update the formula:

```bash
# Update the URL and checksum in the formula
# For releases from tags:
url "https://github.com/yingkitw/datacell/archive/refs/tags/v0.1.6.tar.gz"
sha256 "<checksum>"  # Get this from: sha256sum v0.1.6.tar.gz
```

## Uninstallation

```bash
brew uninstall datacell
brew untap yingkitw/datacell  # If using custom tap
```

## Verification

After installation, verify:

```bash
datacell --version
datacell --help
```

## Troubleshooting

### Rust not found
```bash
brew install rust
```

### Permission errors
```bash
sudo chown -R $(whoami) /usr/local/lib/node_modules
```

### Formula conflicts
```bash
brew unlink datacell  # If another version exists
brew install datacell
```
