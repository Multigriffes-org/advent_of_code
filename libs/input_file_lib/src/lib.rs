use std::{error::Error, fs};

pub fn get_file_content_to_string(path: &String) -> Result<String, Box<dyn Error>> {
    let input_file = path.replace('"', "");
    let content = fs::read_to_string(input_file)?;
    Ok(content)
}