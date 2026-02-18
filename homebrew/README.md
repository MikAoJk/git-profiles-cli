# Homebrew Formula Template

This directory contains a template Homebrew formula for git-profiles-cli.

## What is this?

This `git-profiles-cli.rb` file is a Ruby-based Homebrew formula that tells Homebrew how to install the git-profiles-cli tool on macOS.

## How to use it?

### For Maintainers:

1. **Create a separate Homebrew tap repository** (one-time setup):
   ```bash
   gh repo create MikAoJk/homebrew-tap --public
   ```

2. **Clone and set up the tap**:
   ```bash
   git clone https://github.com/MikAoJk/homebrew-tap.git
   cd homebrew-tap
   mkdir Formula
   ```

3. **Copy this formula to your tap**:
   ```bash
   cp /path/to/git-profiles-cli/homebrew/git-profiles-cli.rb Formula/
   ```

4. **When releasing a new version**, update the formula with:
   - New version number
   - New release URLs
   - New SHA256 checksums (from release assets)

### For Users:

Once the tap is set up, users can install with:
```bash
brew tap MikAoJk/tap
brew install git-profiles-cli
```

## Formula Structure

The formula includes:
- **desc**: Short description of the tool
- **homepage**: Project homepage URL
- **version**: Current version number
- **license**: Software license (MIT)
- **url**: Download URLs for x86_64 and ARM64 binaries
- **sha256**: Checksums for verification
- **install**: Installation instructions
- **test**: Basic smoke test

## Architecture Support

The formula automatically detects the CPU architecture:
- **Apple Silicon (M1/M2/M3)**: Uses `aarch64-apple-darwin` binary
- **Intel Macs**: Uses `x86_64-apple-darwin` binary

## Related Documentation

- [HOMEBREW_QUICKSTART.md](../HOMEBREW_QUICKSTART.md) - Quick start guide
- [RELEASE_PROCESS.md](../RELEASE_PROCESS.md) - Complete release process
- [HOMEBREW_RELEASE.md](../HOMEBREW_RELEASE.md) - Technical reference

## Notes

This formula file is a **template**. It needs to be:
1. Copied to a separate `homebrew-tap` repository
2. Updated with actual SHA256 checksums when releasing
3. Maintained separately from this main repository

The GitHub Actions workflow (`.github/workflows/release.yml`) automatically builds the binaries that this formula downloads.
