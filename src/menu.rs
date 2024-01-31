use std::fmt;

use inquire::{InquireError, Select};

use crate::{git_commands::GitCommands, version::Version, version_manager::VersionManager};

pub enum MenuOption {
    Mayor(String),
    Minor(String),
    Patch(String),
}

impl fmt::Display for MenuOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MenuOption::Mayor(s) => {
                write!(f, "{}", s)
            }
            MenuOption::Minor(s) => {
                write!(f, "{}", s)
            }
            MenuOption::Patch(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

pub struct Menu {
    version_manager: VersionManager,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            version_manager: VersionManager::new(),
        }
    }

    fn get_options() -> Vec<MenuOption> {
        let mut options: Vec<MenuOption> = vec![];
        options.push(MenuOption::Mayor(String::from("Create new mayor version")));
        options.push(MenuOption::Minor(String::from("Create new minor version")));
        options.push(MenuOption::Patch(String::from("Create new patch version")));
        options
    }

    pub fn show_menu(&self) {
        // First we show the title
        self.show_title();

        let options = Menu::get_options();
        let ans: Result<MenuOption, InquireError> =
            Select::new("What do you want to do?", options).prompt();

        let new_version = match ans {
            Ok(choice) => match choice {
                MenuOption::Mayor(_) => self.version_manager.update_mayor_version(),
                MenuOption::Minor(_) => self.version_manager.update_minor_version(),
                MenuOption::Patch(_) => self.version_manager.update_patch_version(),
            },
            Err(_) => panic!("There was an error, please try again"),
        };

        self.confirm_new_version(new_version);
    }

    fn show_title(&self) {
        println!("Welcome to the version manager");
        println!(
            "Your current newer version is: {}",
            self.version_manager.last_version()
        );
    }

    pub fn confirm_new_version(&self, new_version: Version) {
        println!("Your new version will look like this: {}", new_version);
        let options: Vec<&str> = vec!["Yes", "No"];

        let ans: Result<&str, InquireError> =
            Select::new("Do you want to continue?", options).prompt();
        match ans {
            Ok(ch) => {
                if ch == "Yes" {
                    Menu::change_version(new_version);
                }
            }
            Err(_) => println!("Error"),
        }
    }

    fn change_version(new_version: Version) {
        let result = GitCommands::tag_version(new_version.get_version_string());
        match result {
            Ok(_) => {
                println!("Your new version is: {}", new_version);
                println!("To push your new version you should excute:");
                println!("git push origin {}", new_version);
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
    }
}
