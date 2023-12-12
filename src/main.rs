use std::process::Command;

fn main() {
    let out = Command::new("git").arg("tag").output().expect("Error");
    println!("Status: {}", out.status);
    println!("Content string: {}", String::from_utf8_lossy(&out.stdout));
}
