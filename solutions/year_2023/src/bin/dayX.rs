use std::{env, error::Error};

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: dayX <input_file>");
	    std::process::exit(1);
    }

    let string_to_compute = get_file_content_to_string(&args[1])?;

    Ok(())
}