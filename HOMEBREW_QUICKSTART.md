# Quick Start: Homebrew Release

## For First-Time Setup (One Time Only)

1. **Create your Homebrew tap repository:**
   ```bash
   gh repo create MikAoJk/homebrew-tap --public
   git clone https://github.com/MikAoJk/homebrew-tap.git
   cd homebrew-tap
   mkdir Formula
   cp /path/to/git-profiles-cli/homebrew/git-profiles-cli.rb Formula/
   git add .
   git commit -m "Add git-profiles-cli formula"
   git push
   ```

2. **Optional: Set up automated tap updates**
   - Create a GitHub Personal Access Token with `repo` permissions
   - Add it as a secret named `HOMEBREW_TAP_TOKEN` in this repository

## For Every Release

### Quick Steps:
```bash
# 1. Update version in Cargo.toml
vim Cargo.toml  # Change version to 1.0.2

# 2. Commit and push
git add Cargo.toml
git commit -m "Bump version to 1.0.2"
git push

# 3. Create and push tag
git tag -a v1.0.2 -m "Release v1.0.2"
git push origin v1.0.2

# 4. Create GitHub release
# Go to https://github.com/MikAoJk/git-profiles-cli/releases/new
# - Select tag: v1.0.2
# - Add release notes
# - Click "Publish release"

# 5. Wait for GitHub Actions to complete (5-10 minutes)

# 6. Update Homebrew formula
# Download SHA256 files from release assets and update Formula/git-profiles-cli.rb
```

### Automated Option:
If you set up `HOMEBREW_TAP_TOKEN`:
1. Go to Actions → Update Homebrew Tap → Run workflow
2. Enter version and SHA256 values
3. Done!

## Users Can Then Install With:
```bash
brew install MikAoJk/tap/git-profiles-cli
```

---

**Full Documentation:**
- [RELEASE_PROCESS.md](RELEASE_PROCESS.md) - Complete step-by-step guide
- [HOMEBREW_RELEASE.md](HOMEBREW_RELEASE.md) - Technical reference
