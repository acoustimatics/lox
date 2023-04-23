mod reporting;
mod scanning;

use std::env;
use std::error;
use std::fs;
use std::io::{self, Write};
use std::process;

use reporting::Reporter;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut reporter = Reporter::new();
    let return_code = if args.len() > 2 {
        println!("Usage: lox [script]");
        64
    } else if args.len() == 2 {
        run_file(&args[0], &mut reporter)
    } else {
        run_prompt(&mut reporter);
    };
    process::exit(return_code);
}

/// Reads and executes a lox file.
fn run_file(path: &str, reporter: &mut Reporter) -> i32 {
    match fs::read_to_string(path) {
        Ok(code) => {
            run(&code, reporter);
            if reporter.had_error {
                65
            } else {
                0
            }
        }
        Err(e) => {
            reporter.result_error(&e);
            1
        }
    }
}

/// Runs a read/eval/print loop.
fn run_prompt(reporter: &mut Reporter) -> ! {
    loop {
        print!("> ");
        // Must flush or the prompt never gets printed.
        if let Err(e) = io::stdout().flush() {
            reporter.result_error(&e);
            continue;
        }
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            reporter.result_error(&e);
            continue;
        }
        run(&buffer, reporter);
        reporter.had_error = false;
    }
}

/// Executes a lox source text.
fn run(source: &str, reporter: &mut Reporter) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}
