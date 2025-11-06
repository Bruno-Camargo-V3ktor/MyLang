use std::{env::args, process::ExitCode};

use crate::mylang::MyLang;

mod debug;
mod mylang;
mod tokens;

fn main() -> ExitCode {
    let args = args().into_iter().collect::<Vec<String>>();
    let args_len = args.len();

    if args_len > 2 {
        eprintln!("Usage: {} file.my \n", args[0]);
        return ExitCode::FAILURE;
    }

    if args_len == 2 {
        MyLang::run_file(&args[1]);
    } else {
        MyLang::run_prompt();
    }

    ExitCode::SUCCESS
}
