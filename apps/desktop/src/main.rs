pub mod app_state;

use config::config::get_workspace_pathbuf;
use std::process::{Command, Stdio};

fn main() {
    let _state = app_state::AppState::new();

    let workspace_pathbuf = get_workspace_pathbuf().unwrap();
    let file_location = workspace_pathbuf.join("ui/gtk/zig-out/bin/eiyodex-gtk-ui");
    let _ = Command::new(file_location)
        .current_dir(workspace_pathbuf)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped()) // <-- capture stdout
        .spawn()
        .expect("Failed to start UI");

    if cfg!(unix) {
        println!("Started Eiyodex GTK ui for unix");
    } else if cfg!(windows) {
        println!("Started Eiyodex GTK ui for windows");
    } else if cfg!(target_os = "macos") {
        println!("Started Eiyodex GTK ui for macos");
    }
}
