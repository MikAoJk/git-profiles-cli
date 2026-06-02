# git-profiles-cli
A CLI tool to switch between private and work user profiles or other git profiles

![Crates.io Version](https://img.shields.io/crates/v/git-profiles-cli)
[![Snap Store](https://snapcraft.io/git-profiles-cli/badge.svg)](https://snapcraft.io/git-profiles-cli)


## Technologies used
* Rust
* Cargo
* Git

## Installation Methods
### 1. Installing from Snap Store
```bash script
sudo snap install git-profiles-cli
```

### 2. Installing from Crates.io
```bash script
cargo install git-profiles-cli
```
### 3. Installing from a Git repository
```bash script
cargo install --git https://github.com/MikAoJk/git-profiles-cli git-profiles-cli
```

## Using the cli

### Add a profile

```
git-profiles-cli add <name> --user <user.name> --email <user.email>
# Example:
git-profiles-cli add work --user "Work Name" --email work@example.com
```

### List all profiles

```
git-profiles-cli list
```

### Switch to a profile

```
git-profiles-cli switch <name>
# Example:
git-profiles-cli switch work
```

### View current user

```
git-profiles-cli current
```

### Remove a profile

```
git-profiles-cli remove <name>
```

## Where are profiles stored?

Profiles are stored in a config file at this location:
- Linux/macOS: `~/.config/git-profiles-cli/config.toml`

---


### Local development Prerequisites
#### Rust
Make sure you have the correct rust installed, see: [Installing Rust](https://www.rust-lang.org/learn/get-started)
The version of rust used in this project could be found here: [Cargo.toml](Cargo.toml)
To verify the version of rust installed, run the following command:
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
> Note: installing Rust using rustup will also install cargo
```bash script
cargo --version
```

#### Git
Make sure you have git installed using this command:
```bash script
git --version
```

### Build code
Build the code without running it
```bash script
cargo build
```

#### Run the cli
Run the cli
```bash script
cargo run
```
