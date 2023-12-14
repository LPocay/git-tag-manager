use colored::Colorize;
use std::{fmt, process::Command};

use inquire::{InquireError, Select};

struct Version {
    mayor: i32,
    minor: i32,
    patch: i32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formated_colorized = format!("{}.{}.{}", self.mayor, self.minor, self.patch)
            .green()
            .bold();
        write!(f, "{}", formated_colorized)
    }
}

struct VersionManager {
    versions: Vec<Version>,
}

impl VersionManager {
    fn new() -> VersionManager {
        VersionManager { versions: vec![] }
    }

    fn validate_order(&mut self) {
        self.versions.sort_by(|a, b| {
            if a.mayor != b.mayor {
                a.mayor.cmp(&b.mayor)
            } else if a.minor != b.minor {
                a.minor.cmp(&b.minor)
            } else {
                a.patch.cmp(&b.patch)
            }
        })
    }

    fn last_version(&self) -> &Version {
        &self.versions.last().expect("No last")
    }

    fn show_options(&self) {
        let options: Vec<&str> = vec![
            "Create new mayor version",
            "Create new minor version",
            "Create new patch version",
        ];

        let ans: Result<&str, InquireError> =
            Select::new("What do you want to do?", options).prompt();

        match ans {
            Ok(choice) => {
                if choice == "Create new mayor version" {
                    self.update_mayor_version();
                }
                if choice == "Create new minor version" {
                    self.update_minor_version();
                }
                if choice == "Create new patch version" {
                    self.update_patch_version();
                }
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }

    fn change_version(&self, new_version: Version) {
        let out = Command::new("git")
            .arg("tag")
            .arg("-a")
            .arg(format!("{}", new_version))
            .arg("-m")
            .arg(format!("\"Version {}\"", new_version))
            .output()
            .expect("Error");
        if out.status.success() {
            println!("Your new version is: {}", new_version);
            println!("To push your new version you should excute:");
            println!("git push origin {}", new_version);
        } else {
            let string_out = String::from_utf8_lossy(&out.stdout);
            println!("Error escuting command: {}", string_out);
        }
    }

    fn confirm_version(&self, new_version: Version) {
        println!("Your new version will look like this: {}", new_version);
        let options: Vec<&str> = vec!["Yes", "No"];

        let ans: Result<&str, InquireError> =
            Select::new("Do you want to continue?", options).prompt();
        match ans {
            Ok(ch) => {
                if ch == "Yes" {
                    self.change_version(new_version);
                }
            }
            Err(_) => println!("Error"),
        }
    }

    fn update_mayor_version(&self) {
        let last_version = self.last_version();

        let new_mayor_version = Version {
            mayor: last_version.mayor + 1,
            minor: 0,
            patch: 0,
        };

        self.confirm_version(new_mayor_version);
    }

    fn update_minor_version(&self) {
        let last_version = self.last_version();

        let new_mayor_version = Version {
            mayor: last_version.mayor,
            minor: last_version.minor + 1,
            patch: 0,
        };

        self.confirm_version(new_mayor_version);
    }

    fn update_patch_version(&self) {
        let last_version = self.last_version();

        let new_mayor_version = Version {
            mayor: last_version.mayor,
            minor: last_version.minor,
            patch: last_version.patch + 1,
        };

        self.confirm_version(new_mayor_version);
    }
}

/*
* ["1.0.1", "1.2.1", "1.0.14, "2.0.0", "1.3.12", "0.2.1"]
*
*
* */
fn main() {
    let out = Command::new("git").arg("tag").output().expect("Error");
    let mut version_manager = VersionManager::new();
    if out.status.success() {
        let string_out = String::from_utf8_lossy(&out.stdout);
        for line in string_out.lines() {
            let splited: Vec<&str> = line.split(".").collect();
            let mayor: i32 = splited[0].parse().expect("Error parsing version");
            let minor: i32 = splited[1].parse().expect("Error parsing version");
            let patch: i32 = splited[2].parse().expect("Error parsing version");
            let new_version = Version {
                mayor,
                minor,
                patch,
            };
            version_manager.versions.push(new_version);
        }

        version_manager.validate_order();

        println!(
            "Your current newer version is: {}",
            version_manager.last_version()
        );

        version_manager.show_options();
    }
}
