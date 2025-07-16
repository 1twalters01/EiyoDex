use anyhow::Result;
use clap::Parser;

mod args;
mod css;
mod prettier;
mod python;
mod swiftui;
mod xaml;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match cli.command {
        args::Commands::SetupPython => python::setup_python()?,
        args::Commands::RunPrettier => prettier::run_prettier()?,
        args::Commands::BuildCSS => css::build_css_from_scss()?,
        args::Commands::BuildXaml => xaml::build_xaml_from_scss()?,
        args::Commands::BuildSwiftuiModifiers => swiftui::build_swiftui_modifiers_from_scss()?,
    }

    Ok(())
}
