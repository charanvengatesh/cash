use std::env;
use std::io::{ stdin, stdout, Write };
use std::path::Path;

fn main() {
    loop {
        print!("{}> ", env::current_dir().unwrap().display());
        let _ = stdout().flush();

        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();

        if buf == "\n" {
            continue;
        }

        let mut parts = buf.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args
                    .peekable()
                    .peek()
                    .map_or("/", |x| *x);
                let root = Path::new(new_dir);
                println!("{:?}", new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "pwd" => {
                let path = env::current_dir().unwrap();
                println!("{}", path.display());
            }
            "ls" => {
                let path = env::current_dir().unwrap();
                let mut entries = path.read_dir().unwrap();
                println!("{:?}", entries);

                while let Some(entry) = entries.next() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let file_name = path.file_name().unwrap().to_str().unwrap();

                    println!("{}", file_name);
                }
            }
            "exit" => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}
