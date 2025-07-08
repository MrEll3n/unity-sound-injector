mod args;

use args::reader::arg_handler;
use std::io;

fn main() -> io::Result<()> {
    println!("Unity Sound Injector v0.1.0");

    // Argument handler, that takes care of validity of the .asset file
    arg_handler();

    Ok(())
}
