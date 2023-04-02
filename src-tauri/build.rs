use std::process::Command;

fn main() {
    tauri_build::build();

    Command::new("typeshare")
        .arg("--lang=typescript")
        .arg("--output-file=../src/generated/tauri.ts")
        .arg(".")
        .output()
        .expect("Failed to execute command");
}
