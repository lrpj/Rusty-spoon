# Rusty-spoon Todo CLI

A simple command-line todo application written in Rust.

## Prerequisites
- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

## How to Run

1. Open a terminal and navigate to the project directory:
   ```powershell
   cd C:\Users\pinhe\Documents\Informatikk\Rusty-spoon\project
   ```
2. Build and run the program:
   ```powershell
   cargo run -- [COMMAND]
   ```
   Replace `[COMMAND]` with one of the options below.

## Commands

- **Add a new todo**
  ```powershell
  cargo run -- add "Your todo text here"
  ```
- **List all todos**
  ```powershell
  cargo run -- list
  ```
- **Mark a todo as done**
  ```powershell
  cargo run -- done <id>
  ```
  Replace `<id>` with the number of the todo to mark as done.
- **Remove a todo**
  ```powershell
  cargo run -- remove <id>
  ```
  Replace `<id>` with the number of the todo to remove.

## Example Usage
```powershell
cargo run -- add "Buy groceries"
cargo run -- list
cargo run -- done 1
cargo run -- remove 1
```

Todos are saved in `todos.json` in the project directory.
