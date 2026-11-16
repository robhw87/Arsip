use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a file
    Add { name: String },
    /// Remove a file
    Remove { name: String },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { name } => println!("Adding {}", name),
        Commands::Remove { name } => println!("Removing {}", name),
    }
}
