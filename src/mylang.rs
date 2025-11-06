use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path,
};

pub struct MyLang {}

impl MyLang {
    fn run(source: &str) {
        print!("{source}");
        let _ = io::stdout().flush();
    }

    pub fn run_file(path: &str) {
        let path = Path::new(path);
        if !path.is_file() {
            eprintln!("File not found.");
            std::process::exit(66);
        }

        if let Ok(file) = File::open(path) {
            let mut content = String::new();
            let mut reader = BufReader::new(file);
            match reader.read_to_string(&mut content) {
                Ok(_) => {
                    Self::run(&content);
                }

                Err(_) => {
                    eprintln!("Error reading file.");
                    std::process::exit(77);
                }
            }
        } else {
            eprintln!("Permission error opening file.");
            std::process::exit(66);
        }
    }

    pub fn run_prompt() {
        let mut line = String::new();
        let stdin = io::stdin();

        print!("my> ");
        let _ = io::stdout().flush();

        loop {
            if let Ok(_) = stdin.read_line(&mut line) {
                if line == "exit\n" {
                    break;
                }

                Self::run(&line);

                print!("my> ");
                let _ = io::stdout().flush();
            } else {
                break;
            }

            line.clear();
        }
    }
}
