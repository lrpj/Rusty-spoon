use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Define the Todo struct (add here when add new command in main)
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}

const FILE_PATH: &str = "todos.json";

fn load_todos() -> Vec<Todo> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(FILE_PATH).expect("Failed to read file");
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write(FILE_PATH, data).expect("Failed to write file");
}


use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "A simple CLI Todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },
    List,
    Done { id: usize },
}



fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    // Handle commands
    match cli.command {

        // Add a new todo
        Commands::Add { text } => {
            let id = todos.len() + 1;
            todos.push(Todo { id, text, done: false });
            save_todos(&todos);
            println!("Added todo #{id}");
        }

        // List all todos
        Commands::List => {
            if todos.is_empty() {
                println!("📭 No todos yet!");
            } else {
                for todo in &todos {
                    let status = if todo.done { "✅" } else { "❌" };
                    println!("{} [{}] {}", todo.id, status, todo.text);
                }
            }
        }

        // Mark a todo as done
        Commands::Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.done = true;
                save_todos(&todos);
                println!("Marked todo #{id} as done!");
            } else {
                println!("Todo #{id} not found");
            }
        }

        //Remove a todo
        Commands::Remove { id } => {
            let initial_len = todos.len();
            todos.retain(|t| t.id != id);
            if todos.len() < initial_len {
                // Reassign IDs
                for (index, todo) in todos.iter_mut().enumerate() {
                    todo.id = index + 1;
                }
                save_todos(&todos);
                println!("Removed todo #{id}");
            } else {
                println!("Todo #{id} not found");
            }
        }
    }
}

