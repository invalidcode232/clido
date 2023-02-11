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
            match is_done.to_owned().as_str() {
                "t" => is_done_bool = true,
                "true" => is_done_bool = true,
                "f" => is_done_bool = false,
                "false" => is_done_bool = false,
                _ => println!("ss"),
            }

            todo_client.set_done(index.to_owned(), is_done_bool)
        }
        None => (),
    }
}
