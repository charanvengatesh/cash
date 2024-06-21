# Simple Shell in Rust

This Rust application is a simple shell that accepts basic commands like `cd`, `pwd`, `ls`, and `exit`. It's designed to demonstrate how to interact with the system's environment, file system, and processes in Rust.

## Features

- **Change Directory (`cd`)**: Changes the current working directory of the shell.
- **Print Working Directory (`pwd`)**: Prints the current working directory.
- **List (`ls`)**: Lists the contents of the current directory.
- **Exit (`exit`)**: Exits the shell.

## How to Run

1. Ensure you have Rust installed on your system. If not, you can install it from [the official website](https://www.rust-lang.org/tools/install).
2. Clone this repository or copy the source code into a `.rs` file.
3. Navigate to the directory containing the source code in your terminal.
4. Compile the code with `rustc <filename>.rs`.
5. Run the compiled executable with `./<filename>` on Unix/Linux/macOS or `<filename>.exe` on Windows.

## Usage

After running the application, you will be presented with a prompt (`>`). You can type any of the supported commands (`cd`, `pwd`, `ls`, `exit`) followed by Enter.

Example:
```bash
pwd 
/Users/username/projects/rust-shell 
```


## Limitations

- The `ls` command currently prints raw metadata about directory entries, which might be less readable compared to the standard `ls` command output.
- Error handling is minimal, primarily using `unwrap()` for simplicity, which may cause the program to panic on errors instead of handling them gracefully.

