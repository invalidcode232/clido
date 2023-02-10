pub mod todo;
use clap::{Parser, Subcommand};
use todo::client;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { todo: String },
    List {},
}

fn main() {
    // Initialize our todo client with default values
    // TODO: Replace with the real path name
    let mut todo_client = client::TodoClient {
        ..Default::default()
    };

    todo_client.init();

    // Initialize our Clap cli parser
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { todo }) => {
            todo_client.add(todo);
        }
        Some(Commands::List {}) => {
            todo_client.list();
        }
        None => (),
    }
}
