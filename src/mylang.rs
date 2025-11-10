use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path, process::ExitCode,
};

pub struct MyLang {}

impl MyLang {
    fn run(source: &str) -> Result<(), ExitCode> {
        print!("{source}");
        let _ = io::stdout().flush();

        Ok(())
    }

    pub fn run_file(path: &str) -> Result<(), ExitCode> {
        let path = Path::new(path);
        if !path.is_file() {
            eprintln!("File not found.");
            return Err(ExitCode::from(66));
        }

        if let Ok(file) = File::open(path) {
            let mut content = String::new();
            let mut reader = BufReader::new(file);
            match reader.read_to_string(&mut content) {
                Ok(_) => {
                    Self::run(&content)?;
                    return Ok(())
                }

                Err(_) => {
                    eprintln!("Error reading file.");
                    return Err(ExitCode::from(77));
                }
            }
        } else {
            eprintln!("Permission error opening file.");
            return Err(ExitCode::from(66));
        }
    }

    pub fn run_prompt() -> Result<(), ExitCode>{
        let mut line = String::new();
        let stdin = io::stdin();

        print!("my> ");
        let _ = io::stdout().flush();

        loop {
            if let Ok(_) = stdin.read_line(&mut line) {
                
                if line.trim() == "exit" {
                    break;
                }

                Self::run(&line)?;

                print!("my> ");
                let _ = io::stdout().flush();
            } else {
                break;
            }

            line.clear();
        }

        Ok(())
    }
}
