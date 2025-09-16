# git-profiles-cli
A CLI tool to switch between private and work user profiles

![Crates.io Version](https://img.shields.io/crates/v/git-profiles-cli)


## Technologies used
* Rust
* Cargo

## Installation Methods
### 1. Installing from Crates.io
```bash script
cargo install git-profiles-cli
```
### 2. Installing from a Git repository
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

```[LICENSE](../bible-group-meeting-notification-sms/LICENSE)
git-profiles-cli current
```

### Remove a profile

```
git-profiles-cli remove <name>
```

## Where are profiles stored?

Profiles are stored in a config file at thid location:
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

### Build code
Build the code without running it
```bash script
cargo build
```

#### Run the application locally
##### Run code
Run the application locally
```bash script
cargo run
```
