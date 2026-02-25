# Releasing to the Snap Store

This document describes the process for releasing git-profiles-cli to the Snap Store.

## Prerequisites

Before you can publish to the Snap Store, you need to:

1. **Create a Snapcraft developer account**
   - Visit https://snapcraft.io/
   - Click "Sign in" and create an account (you can use Ubuntu SSO)

2. **Register the snap name**
   - Once logged in, go to https://snapcraft.io/snaps
   - Click "Register a snap name"
   - Register the name `git-profiles-cli`

3. **Generate Snapcraft credentials**
   - Install snapcraft on your local machine: `sudo snap install snapcraft --classic`
   - Export your credentials: `snapcraft export-login snapcraft-credentials`
   - This will create a file with your credentials

4. **Add credentials to GitHub Secrets**
   - Go to your repository on GitHub
   - Navigate to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `SNAPCRAFT_STORE_CREDENTIALS`
   - Value: Paste the contents of the `snapcraft-credentials` file
   - Click "Add secret"

## Automated Release Process

The snap is automatically built and published when you create a new GitHub release:

1. **Create a new release**
   - Go to the repository on GitHub
   - Click "Releases" → "Create a new release"
   - Create a new tag (e.g., `v1.0.2`)
   - Fill in the release title and description
   - Click "Publish release"

2. **Workflow execution**
   - The GitHub Actions workflow `.github/workflows/publish-snap.yml` will automatically:
     - Build the snap for multiple architectures (amd64, arm64, armhf)
     - Publish it to the Snap Store in the stable channel
   - You can monitor the progress in the "Actions" tab

3. **Verify publication**
   - After the workflow completes, visit https://snapcraft.io/git-profiles-cli
   - Your snap should be available in the stable channel
   - Users can now install it with: `sudo snap install git-profiles-cli`

## Manual Testing (Optional)

If you want to test the snap build locally before releasing:

### Install snapcraft
```bash
sudo snap install snapcraft --classic
```

### Build the snap locally
```bash
# From the repository root
snapcraft
```

This will create a `.snap` file in the current directory.

### Install the snap locally
```bash
sudo snap install --dangerous ./git-profiles-cli_*.snap
```

### Test the installed snap
```bash
git-profiles-cli --help
git-profiles-cli list
```

### Clean up
```bash
sudo snap remove git-profiles-cli
snapcraft clean
```

## Snap Configuration

The snap configuration is defined in `snap/snapcraft.yaml`:

- **Base**: core22 (Ubuntu 22.04 LTS)
- **Confinement**: strict (for security)
- **Plugs**: 
  - `home` - allows access to user's home directory for config storage
  - `network` - allows network access if needed
- **Architectures**: amd64, arm64, armhf

## Troubleshooting

### Build failures
- Check the GitHub Actions logs for detailed error messages
- Ensure all dependencies are properly declared in `snapcraft.yaml`
- Test the build locally with `snapcraft`

### Permission issues
- Verify that `SNAPCRAFT_STORE_CREDENTIALS` secret is properly set
- Ensure the snap name is registered to your account
- Check that your credentials haven't expired

### Snap installation issues
- Users may need to connect interfaces: `sudo snap connect git-profiles-cli:home`
- Check snap logs: `snap logs git-profiles-cli`

## Channels

Snaps support different release channels:

- **stable** - Production releases (default in the workflow)
- **candidate** - Release candidates for testing
- **beta** - Beta releases
- **edge** - Development builds

To publish to a different channel, modify the `release` parameter in `.github/workflows/publish-snap.yml`.

## Additional Resources

- [Snapcraft documentation](https://snapcraft.io/docs)
- [Snapcraft Rust plugin](https://snapcraft.io/docs/rust-plugin)
- [GitHub Actions for Snapcraft](https://github.com/snapcore/action-build)
