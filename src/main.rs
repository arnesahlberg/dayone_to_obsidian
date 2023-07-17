use std::{fs::File, io::Read};
use std::error::Error;
pub mod types;
use types::Journal;
pub mod converter;
use converter::convert_to_obsidian;


fn main() -> Result<(), Box<dyn Error>> {
    let input_folder = "/Users/arnesahlberg/Downloads/07-16-2023_9-33-em";
    let output_folder = "/Users/arnesahlberg/Downloads/07-16-2023_9-33-em-obsidian";
    convert_to_obsidian(input_folder, output_folder, true)?;
    Ok(())
}
