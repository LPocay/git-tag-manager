use menu::Menu;

pub mod git_commands;
pub mod menu;
pub mod version;
pub mod version_manager;

fn main() {
    let menu = Menu::new();
    menu.show_menu();
}
