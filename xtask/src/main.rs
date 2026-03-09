use anyhow::Result;
use clap::Parser;

mod args;
mod css;
mod migrations;
mod prettier;
mod python;
mod swiftui;
mod xaml;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match cli.command {
        args::Commands::SetupPython => python::setup_python()?,
        args::Commands::RunPrettier => prettier::run_prettier()?,
        args::Commands::RunMigrations { env } => migrations::run_database_migrations(&env.to_string()).await.unwrap(),
        args::Commands::BuildCSS => css::build_css_from_scss()?,
        args::Commands::BuildXaml => xaml::build_xaml_from_scss()?,
        args::Commands::BuildSwiftuiModifiers => swiftui::build_swiftui_modifiers_from_scss()?,
    }

    Ok(())
}
