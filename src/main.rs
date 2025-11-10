use std::{env::args, process::ExitCode};
use crate::mylang::MyLang;

mod debug;
mod mylang;
mod tokens;
mod scanner;

fn main() -> Result<(), ExitCode> {
    let args = args().into_iter().collect::<Vec<String>>();
    let args_len = args.len();

    if args_len > 2 {
        eprintln!("Usage: {} file.my", args[0]);
        return Err(ExitCode::FAILURE);
    }

    if args_len == 2 {
        MyLang::run_file(&args[1])?;
    } else {
        MyLang::run_prompt()?;
    }

    Ok(())
}
