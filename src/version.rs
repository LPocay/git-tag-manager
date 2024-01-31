use colored::Colorize;
use std::fmt;

pub struct Version {
    pub mayor: i32,
    pub minor: i32,
    pub patch: i32,
}

impl Version {
    pub fn get_version_string(&self) -> String {
        format!("{}.{}.{}", self.mayor, self.minor, self.patch)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formated_colorized = format!("{}.{}.{}", self.mayor, self.minor, self.patch)
            .green()
            .bold();
        write!(f, "{}", formated_colorized)
    }
}
