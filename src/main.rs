use std::{fmt, process::Command};

struct Version {
    mayor: i32,
    minor: i32,
    patch: i32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.mayor, self.minor, self.patch)
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
        &self.versions[0]
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
            "Your current newer version: {}",
            version_manager.last_version()
        );
    }
}
