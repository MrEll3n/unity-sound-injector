use std::{env, process};

/// Returns vector of argument strings without program name
fn read_arguments() -> Vec<String> {
    env::args().skip(1).collect()
}

fn has_arguments(target: &str) -> bool {
    env::args().skip(1).any(|arg| arg == target)
}

fn print_help() {
    println!("Usage: usj <file.asset>");
}

// Main argument handler
pub fn arg_handler() {
    // args without the name of the program
    let args: Vec<String> = read_arguments();

    match args.len() {
        1 => {}
        _ => {
            print_help();
            process::exit(1);
        }
    }
}
