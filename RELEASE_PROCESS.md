# Complete Guide: Setting Up Homebrew Releases

This guide walks through the entire process of setting up Homebrew releases for git-profiles-cli.

## Table of Contents
1. [Overview](#overview)
2. [One-Time Setup](#one-time-setup)
3. [Creating a Release](#creating-a-release)
4. [Updating the Homebrew Tap](#updating-the-homebrew-tap)
5. [Testing](#testing)
6. [Troubleshooting](#troubleshooting)

## Overview

The release process consists of three main steps:
1. **Build binaries**: GitHub Actions automatically builds and packages binaries for macOS and Linux
2. **Create GitHub release**: Tag and publish a release on GitHub
3. **Update Homebrew formula**: Update the formula in your Homebrew tap with new version and checksums

## One-Time Setup

### Step 1: Create a Homebrew Tap Repository

1. Create a new GitHub repository named `homebrew-tap`:
   ```bash
   # Using GitHub CLI
   gh repo create MikAoJk/homebrew-tap --public
   
   # Or manually create at https://github.com/new
   ```

2. Clone and set up the repository structure:
   ```bash
   git clone https://github.com/MikAoJk/homebrew-tap.git
   cd homebrew-tap
   mkdir Formula
   ```

3. Copy the formula template from this repository:
   ```bash
   cp /path/to/git-profiles-cli/homebrew/git-profiles-cli.rb Formula/
   ```

4. Commit and push:
   ```bash
   git add Formula/git-profiles-cli.rb
   git commit -m "Add git-profiles-cli formula"
   git push
   ```

### Step 2: Create a Personal Access Token (Optional)

If you want to use the automated tap update workflow, create a GitHub Personal Access Token:

1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Give it a descriptive name (e.g., "Homebrew Tap Updates")
4. Select scopes:
   - `repo` (Full control of private repositories)
5. Generate and copy the token
6. Add it as a secret in your `git-profiles-cli` repository:
   - Go to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `HOMEBREW_TAP_TOKEN`
   - Value: Paste your token

## Creating a Release

### Step 1: Prepare for Release

1. Ensure all changes are committed and pushed
2. Update the version in `Cargo.toml` if needed:
   ```toml
   [package]
   version = "1.0.2"  # Update this
   ```
3. Update `CHANGELOG.md` with release notes (if you have one)
4. Commit version changes:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 1.0.2"
   git push
   ```

### Step 2: Create and Push a Git Tag

```bash
# Create a tag with the version number
git tag -a v1.0.2 -m "Release version 1.0.2"

# Push the tag to GitHub
git push origin v1.0.2
```

### Step 3: Create a GitHub Release

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Select the tag you just created (`v1.0.2`)
4. Fill in the release title (e.g., "v1.0.2")
5. Add release notes describing changes
6. Click "Publish release"

### Step 4: Wait for GitHub Actions

The release workflow will automatically:
- Build binaries for multiple platforms
- Package them as `.tar.gz` files
- Calculate SHA256 checksums
- Upload everything as release assets

Wait for the workflow to complete (usually 5-10 minutes). You can monitor it under the "Actions" tab.

## Updating the Homebrew Tap

Once the release workflow completes, you need to update your Homebrew formula.

### Option A: Manual Update

1. Go to your release page and download the SHA256 files:
   - `git-profiles-cli-1.0.2-x86_64-apple-darwin.tar.gz.sha256`
   - `git-profiles-cli-1.0.2-aarch64-apple-darwin.tar.gz.sha256`

2. Clone your tap repository (if not already):
   ```bash
   git clone https://github.com/MikAoJk/homebrew-tap.git
   cd homebrew-tap
   ```

3. Edit `Formula/git-profiles-cli.rb`:
   ```ruby
   class GitProfilesCli < Formula
     desc "A CLI tool to switch between private and work user profiles"
     homepage "https://github.com/MikAoJk/git-profiles-cli"
     version "1.0.2"  # ← Update this
     license "MIT"

     if Hardware::CPU.arm?
       url "https://github.com/MikAoJk/git-profiles-cli/releases/download/v1.0.2/git-profiles-cli-1.0.2-aarch64-apple-darwin.tar.gz"  # ← Update version
       sha256 "PASTE_ARM64_SHA256_HERE"  # ← Paste SHA from .sha256 file
     else
       url "https://github.com/MikAoJk/git-profiles-cli/releases/download/v1.0.2/git-profiles-cli-1.0.2-x86_64-apple-darwin.tar.gz"  # ← Update version
       sha256 "PASTE_X86_64_SHA256_HERE"  # ← Paste SHA from .sha256 file
     end

     def install
       bin.install "git-profiles-cli"
     end

     test do
       system "#{bin}/git-profiles-cli", "--help"
     end
   end
   ```

4. Commit and push:
   ```bash
   git add Formula/git-profiles-cli.rb
   git commit -m "Update git-profiles-cli to v1.0.2"
   git push
   ```

### Option B: Automated Update (Requires Setup)

If you've set up the `HOMEBREW_TAP_TOKEN` secret:

1. Go to the "Actions" tab in your `git-profiles-cli` repository
2. Select "Update Homebrew Tap" workflow
3. Click "Run workflow"
4. Fill in:
   - Version: `1.0.2`
   - x86_64 SHA256: (copy from release assets)
   - ARM64 SHA256: (copy from release assets)
5. Click "Run workflow"

The workflow will automatically update your tap and push the changes.

## Testing

### Test the Formula Before Publishing

```bash
# Clone your tap locally
brew tap-new local/test
cd $(brew --repository)/Library/Taps/local/homebrew-test
cp /path/to/homebrew-tap/Formula/git-profiles-cli.rb Formula/

# Audit the formula
brew audit --strict Formula/git-profiles-cli.rb

# Test installation
brew install --build-from-source git-profiles-cli

# Test the binary
git-profiles-cli --help

# Clean up
brew uninstall git-profiles-cli
brew untap local/test
```

### Test User Installation

```bash
# Add your tap
brew tap MikAoJk/tap

# Install
brew install git-profiles-cli

# Verify
git-profiles-cli --version
git-profiles-cli --help

# Test functionality
git-profiles-cli list
```

## Troubleshooting

### Binary Not Found After Installation

**Problem**: Running `git-profiles-cli` shows "command not found"

**Solution**: 
- Verify the binary is in the tar.gz at the root level:
  ```bash
  tar -tzf git-profiles-cli-1.0.2-x86_64-apple-darwin.tar.gz
  ```
- Should show: `git-profiles-cli` (not nested in folders)

### SHA256 Mismatch

**Problem**: Homebrew installation fails with SHA256 mismatch

**Solution**:
1. Download the release asset
2. Calculate SHA256 manually:
   ```bash
   shasum -a 256 git-profiles-cli-1.0.2-x86_64-apple-darwin.tar.gz
   ```
3. Update formula with the correct hash

### Formula Syntax Errors

**Problem**: Formula fails to install due to syntax errors

**Solution**:
```bash
# Validate formula syntax
brew audit --strict Formula/git-profiles-cli.rb

# Check for common issues:
# - Missing `end` statements
# - Incorrect indentation
# - Invalid Ruby syntax
```

### Release Workflow Failed

**Problem**: GitHub Actions workflow fails to build

**Solution**:
1. Check the Actions tab for error logs
2. Common issues:
   - Missing Rust toolchain
   - Build errors (run `cargo build` locally first)
   - Permission issues (check GITHUB_TOKEN)

### Tap Not Found

**Problem**: `brew tap MikAoJk/tap` shows "Error: Invalid tap name"

**Solution**:
- Ensure repository is named `homebrew-tap` (not just `tap`)
- Repository must be public
- Use full format: `username/tap` (e.g., `MikAoJk/tap`)

## Additional Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Acceptable Formulae](https://docs.brew.sh/Acceptable-Formulae)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Cargo Build Documentation](https://doc.rust-lang.org/cargo/commands/cargo-build.html)

## Quick Reference

```bash
# Release checklist
□ Update Cargo.toml version
□ Commit and push changes
□ Create and push git tag
□ Create GitHub release
□ Wait for release workflow to complete
□ Download SHA256 checksums from release
□ Update Homebrew formula
□ Test installation
□ Announce release
```
