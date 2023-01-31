pub mod todo;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { todo: String },
}

fn main() {
    let cli = Cli::parse();

    let mut todo_client = todo::Todo {
        ..Default::default()
    };

    todo_client.init();

    match &cli.command {
        Some(Commands::Add { todo }) => {
            todo_client.write(todo);
        }
        None => (),
    }
}
