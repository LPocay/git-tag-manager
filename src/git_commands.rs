use std::process::Command;

pub struct GitCommands {}

impl GitCommands {
    pub fn get_all_versions() -> Result<String, &'static str> {
        let out = Command::new("git").arg("tag").output().expect("Error");
        if out.status.success() {
            let string_out = String::from_utf8_lossy(&out.stdout);
            Ok(string_out.to_string())
        } else {
            Err("Couldtn't get all versions")
        }
    }

    pub fn tag_version(version_string: String) -> Result<(), String> {
        let out = Command::new("git")
            .arg("tag")
            .arg("-a")
            .arg(&version_string)
            .arg("-m")
            .arg(format!("\"Version {}\"", &version_string))
            .output()
            .expect("Error");
        if out.status.success() {
            Ok(())
        } else {
            let string_out = String::from_utf8_lossy(&out.stderr);
            Err(format!("Error escuting command: {}", string_out))
        }
    }

    pub fn push_tag(version_string: String) -> Result<(), String> {
        let out = Command::new("git")
            .arg("push")
            .arg("origin")
            .arg(&version_string)
            .output()
            .expect("Error");
        if out.status.success() {
            Ok(())
        } else {
            let string_out = String::from_utf8_lossy(&out.stderr);
            Err(format!("Error escuting command: {}", string_out))
        }
    }
}
