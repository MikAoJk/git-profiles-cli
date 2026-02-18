# Homebrew Release Guide

This document explains how to release `git-profiles-cli` to Homebrew.

## Overview

The project uses GitHub Actions to automate building and releasing binaries for macOS and Linux. These binaries can then be installed via Homebrew.

## Prerequisites

1. A GitHub repository for your Homebrew tap (e.g., `MikAoJk/homebrew-tap`)
2. GitHub token with appropriate permissions to update the tap repository

## Setup

### 1. Create a Homebrew Tap Repository

Create a new GitHub repository named `homebrew-tap` (or `homebrew-<custom-name>`). This repository will contain your Homebrew formulas.

```bash
mkdir homebrew-tap
cd homebrew-tap
mkdir Formula
```

### 2. Add the Formula

Copy the formula file from `homebrew/git-profiles-cli.rb` in this repository to `Formula/git-profiles-cli.rb` in your tap repository.

### 3. Create GitHub Secrets

In your main repository (git-profiles-cli), add the following secret:
- `HOMEBREW_TAP_TOKEN`: A personal access token with `repo` permissions to update your tap repository

## Release Process

### Automated Release

When you create a new release on GitHub:

1. Tag the release with a version number (e.g., `v1.0.2`)
2. Publish the release
3. The GitHub Actions workflow will automatically:
   - Build binaries for macOS (x86_64 and ARM64) and Linux
   - Package them as tar.gz archives
   - Upload them as release assets
   - Generate SHA256 checksums

### Updating the Homebrew Formula

After the release workflow completes, update your Homebrew tap:

1. Download the SHA256 checksums from the release assets
2. Update `Formula/git-profiles-cli.rb` in your tap repository:
   - Update the `version` field
   - Update both `url` fields with the new version
   - Replace `sha256` values with the checksums from the release

Example formula update:

```ruby
class GitProfilesCli < Formula
  desc "A CLI tool to switch between private and work user profiles"
  homepage "https://github.com/MikAoJk/git-profiles-cli"
  version "1.0.2"  # Update this
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/MikAoJk/git-profiles-cli/releases/download/v1.0.2/git-profiles-cli-1.0.2-aarch64-apple-darwin.tar.gz"  # Update version
    sha256 "NEW_ARM64_SHA256_HERE"  # Update this
  else
    url "https://github.com/MikAoJk/git-profiles-cli/releases/download/v1.0.2/git-profiles-cli-1.0.2-x86_64-apple-darwin.tar.gz"  # Update version
    sha256 "NEW_X86_64_SHA256_HERE"  # Update this
  end

  def install
    bin.install "git-profiles-cli"
  end

  test do
    system "#{bin}/git-profiles-cli", "--help"
  end
end
```

### Automated Tap Updates (Optional)

To fully automate the tap update process, you can add a workflow that:
1. Triggers after the release workflow completes
2. Checks out your tap repository
3. Updates the formula file programmatically
4. Commits and pushes the changes

This would require the `HOMEBREW_TAP_TOKEN` secret mentioned in the setup.

## Installation for Users

Once the tap is set up, users can install the CLI tool via Homebrew:

```bash
# Add the tap
brew tap MikAoJk/tap

# Install the tool
brew install git-profiles-cli
```

Or in one command:

```bash
brew install MikAoJk/tap/git-profiles-cli
```

## Testing the Formula Locally

Before publishing, you can test the formula locally:

```bash
# Audit the formula
brew audit --strict Formula/git-profiles-cli.rb

# Test installation locally
brew install --build-from-source Formula/git-profiles-cli.rb

# Test the installed binary
git-profiles-cli --help

# Uninstall
brew uninstall git-profiles-cli
```

## Troubleshooting

### Binary not found
- Ensure the tar.gz archive contains the binary at the root level
- Verify the `bin.install` path matches the binary name in the archive

### SHA256 mismatch
- Download the release asset and verify the SHA256:
  ```bash
  shasum -a 256 git-profiles-cli-1.0.1-x86_64-apple-darwin.tar.gz
  ```
- Update the formula with the correct hash

### Architecture detection
- The formula uses `Hardware::CPU.arm?` to detect Apple Silicon
- Test on both Intel and ARM Macs if possible

## References

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Tap Documentation](https://docs.brew.sh/Taps)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
