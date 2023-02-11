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
    Done { index: i32, is_done: String },
    Remove { index: i32 },
}

fn main() {
    // Initialize our todo client with default values
    let path = dirs::home_dir()
        .unwrap()
        .join(".local/share/clido/todo.csv");

    let mut todo_client = client::TodoClient {
        path: path.as_path(),
        ..Default::default()
    };

    // Init function to create todo file and set some variables
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
        Some(Commands::Done { index, is_done }) => {
            let mut is_done_bool = true;
            // Convert string to bool
            match is_done.trim().to_lowercase().as_str() {
                "t" => is_done_bool = true,
                "true" => is_done_bool = true,
                "f" => is_done_bool = false,
                "false" => is_done_bool = false,
                _ => println!("invalid input, expected boolean."),
            }

            todo_client.set_done(index.to_owned(), is_done_bool)
        }
        Some(Commands::Remove { index }) => todo_client.remove_todo(index.to_owned()),
        None => (),
    }
}
