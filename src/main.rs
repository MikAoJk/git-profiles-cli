pub mod config;

use clap::{Parser, Subcommand};

use std::io::{self, Write};
use std::process::Command;
use crate::config::{Config, Profile};

#[derive(Parser)]
#[command(name = "git-profiles-cli")]
#[command(about = "A CLI tool to switch between private and work user profiles")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        email: String,
    },
    Remove {
        name: String,
    },
    List,
    Switch {
        name: String,
    },
    Current,
    Version,
}


fn git_config_set(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--global", key, value])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to set {}: {}",
            key,
            String::from_utf8_lossy(&output.stderr)
        )
            .into());
    }

    Ok(())
}

fn git_config_get(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--global", key])
        .output()?;

    if !output.status.success() {
        return Ok(String::from("Not set"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    match cli.command {
        Commands::Add { name, user, email } => {
            if config.profiles.contains_key(&name) {
                print!("Profile '{}' already exists. Overwrite? [y/N]: ", name);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            config.profiles.insert(
                name.clone(),
                Profile {
                    name: user.clone(),
                    email: email.clone(),
                },
            );

            config.save()?;
            println!("✅ Profile '{}' added successfully", name);
            println!("   Name: {}", user);
            println!("   Email: {}", email);
        }

        Commands::Remove { name } => {
            if config.profiles.remove(&name).is_some() {
                config.save()?;
                println!("✅ Profile '{}' removed successfully", name);
            } else {
                println!("❌ Profile '{}' not found", name);
            }
        }

        Commands::List => {
            if config.profiles.is_empty() {
                println!("No profiles configured.");
                println!(
                    "\nUse 'git-profiles-cli add <name> --user <user> --email <email>' to add a profile."
                );
            } else {
                println!("Configured profiles:");
                println!();

                let mut profiles: Vec<_> = config.profiles.iter().collect();
                profiles.sort_by_key(|(name, _)| name.as_str());

                for (profile_name, profile) in profiles {
                    println!("📋 {}", profile_name);
                    println!("   Name: {}", profile.name);
                    println!("   Email: {}", profile.email);
                    println!();
                }
            }
        }

        Commands::Version => {
            println!(env!("CARGO_PKG_VERSION"));
        }

        Commands::Switch { name } => match config.profiles.get(&name) {
            Some(profile) => {
                git_config_set("user.name", &profile.name)?;
                git_config_set("user.email", &profile.email)?;

                println!("✅ Switched to profile '{}'", name);
                println!("   Name: {}", profile.name);
                println!("   Email: {}", profile.email);
            }
            None => {
                println!("❌ Profile '{}' is not found!", name);
                println!("\nFound these profiles:");
                for profile_name in config.profiles.keys() {
                    println!("  - {}", profile_name);
                }
            }
        },

        Commands::Current => {
            let name = git_config_get("user.name")?;
            let email = git_config_get("user.email")?;

            println!("Current Git user:");
            println!("  Name: {}", name);
            println!("  Email: {}", email);

            for (profile_name, profile) in &config.profiles {
                if profile.name == name && profile.email == email {
                    println!("\n  (Profile: {})", profile_name);
                    break;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_profile() {
        let mut config = Config::default();
        let profile = Profile { name: "Test User".to_string(), email: "test@example.com".to_string() };
        config.profiles.insert("test".to_string(), profile.clone());

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles.get("test").unwrap(), &profile);
    }

    #[test]
    fn test_remove_profile() {
        let mut config = Config::default();
        config.profiles.insert(
            "test".to_string(),
            Profile { name: "Test User".to_string(), email: "test@example.com".to_string() }
        );

        let removed = config.profiles.remove("test");
        assert!(removed.is_some());
        assert!(config.profiles.get("test").is_none());
    }

    #[test]
    fn test_list_profiles() {
        let mut config = Config::default();
        config.profiles.insert(
            "test".to_string(),
            Profile { name: "Test User".to_string(), email: "test@example.com".to_string() }
        );

        let mut profiles: Vec<_> = config.profiles.iter().collect();
        profiles.sort_by_key(|(name, _)| name.as_str());

        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].0, "test");
    }

    #[test]
    fn test_switch_profile_sets_git_config() {
        let profile = Profile { name: "Test User".to_string(), email: "test@example.com".to_string() };
        let result_name = git_config_set("user.name", &profile.name);
        let result_email = git_config_set("user.email", &profile.email);

        assert!(result_name.is_ok());
        assert!(result_email.is_ok());
    }

    #[test]
    fn test_git_config_get_returns_value_or_not_set() {
        let result = git_config_get("user.name");
        assert!(result.is_ok());
    }
 
}
