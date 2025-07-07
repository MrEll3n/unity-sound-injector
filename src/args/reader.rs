use std::env;

/// Returns vector of argument strings
pub fn read_arguments() -> Vec<String> {
    env::args().skip(1).collect()
}

pub fn has_arguments(target: &str) -> bool {
    env::args().skip(1).any(|arg| arg == target)
}

pub fn print_help() {g
    println!("Usage: usj <input1> <input2> <output>");
    println!("Example:");
    println!("  usj input.txt config.json output.log");
    println!();
    println!("Arguments:");
    println!("  <input1>    Path to the input file");
    println!("  <input2>    Path to the config file");
    println!("  <output>    Path to the output file to be created");
}
