use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SetupPython,
    BuildCSS,
    BuildXaml,
    BuildSwiftuiModifiers,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::SetupPython => setup_python()?,
        Commands::BuildCSS => build_css()?,
        Commands::BuildXaml => build_xaml()?,
        Commands::BuildSwiftuiModifiers => build_swiftui_modifiers()?,
    }

    Ok(())
}

fn setup_python() -> Result<()> {
    println!("Setup Python");
    Ok(())
}

fn build_css() -> Result<()> {
    println!("Build CSS for GTK 4 on Linux");
    Ok(())
}

fn build_xaml() -> Result<()> {
    println!("Build XAML for WinUI 3 on Windows");
    Ok(())
}

fn build_swiftui_modifiers() -> Result<()> {
    println!("Build SwiftUI modifiers for SwiftUI on Mac");
    Ok(())
}
