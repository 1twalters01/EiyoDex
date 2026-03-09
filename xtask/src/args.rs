use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    SetupPython,
    RunPrettier,
    RunMigrations {
        #[arg(value_enum)]
        env: Environment,
    },
    BuildCSS,
    BuildXaml,
    BuildSwiftuiModifiers,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Dev => "dev".to_string(),
            Environment::Staging => "staging".to_string(),
            Environment::Prod => "prod".to_string(),
        }
    }
}
