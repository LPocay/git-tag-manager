use crate::git_commands::GitCommands;
use crate::version::Version;

pub struct VersionManager {
    pub versions: Vec<Version>,
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionManager {
    pub fn new() -> VersionManager {
        let versions_string = GitCommands::get_all_versions();
        let mut versions: Vec<Version> = vec![];
        match versions_string {
            Ok(versions_string) => {
                for version_string in versions_string.lines() {
                    let splited: Vec<&str> = version_string.split('.').collect();
                    if splited.len() != 3 {
                        continue;
                    }
                    let mayor: i32 = splited[0].parse().expect("Error parsing version");
                    let minor: i32 = splited[1].parse().expect("Error parsing version");
                    let patch: i32 = splited[2].parse().expect("Error parsing version");
                    let new_version = Version {
                        mayor,
                        minor,
                        patch,
                    };
                    versions.push(new_version);
                }
            }
            Err(_) => {}
        }

        VersionManager::validate_order(&mut versions);
        VersionManager { versions }
    }

    fn validate_order(versions: &mut Vec<Version>) {
        versions.sort_by(|a, b| {
            if a.mayor != b.mayor {
                a.mayor.cmp(&b.mayor)
            } else if a.minor != b.minor {
                a.minor.cmp(&b.minor)
            } else {
                a.patch.cmp(&b.patch)
            }
        })
    }

    pub fn last_version(&self) -> &Version {
        self.versions.last().expect("No last")
    }

    pub fn update_mayor_version(&self) -> Version {
        let last_version = self.last_version();

        let new_mayor_version = Version {
            mayor: last_version.mayor + 1,
            minor: 0,
            patch: 0,
        };

        new_mayor_version
    }

    pub fn update_minor_version(&self) -> Version {
        let last_version = self.last_version();

        let new_mayor_version = Version {
            mayor: last_version.mayor,
            minor: last_version.minor + 1,
            patch: 0,
        };

        new_mayor_version
    }

    pub fn update_patch_version(&self) -> Version {
        let last_version = self.last_version();

        let new_mayor_version = Version {
            mayor: last_version.mayor,
            minor: last_version.minor,
            patch: last_version.patch + 1,
        };

        new_mayor_version
    }
}
