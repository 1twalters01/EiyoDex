use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

fn main() {
    let mut ui = Command::new("./ui/gtk/zig-out/bin/eiyodex-gtk-ui")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())  // <-- capture stdout
        .spawn()
        .expect("Failed to start UI");

    let stdout = ui.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let msg = line.unwrap();
        println!("Received from UI: {}", msg);

        if msg.contains("button_clicked") {
            println!("Button clicked");
        }
    }
}
