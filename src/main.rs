use std::{fs::File, io::Read};
use std::error::Error;
pub mod types;
use types::Journal;
pub mod converter;
use converter::convert_to_obsidian;


fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "/Users/arnesahlberg/Downloads/07-16-2023_9-33-em/Journal.json";
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let journal = serde_json::from_str::<Journal>(&contents)?;
   convert_to_obsidian(journal, "~/Downloads/test".to_string(), false)? ;
    Ok(())
}
