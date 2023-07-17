use std::{fs::File, io::Read};
use std::error::Error;
pub mod types;
use types::Journal;



fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "/Users/arnesahlberg/Downloads/07-16-2023_9-33-em/Journal.json";
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let journal = serde_json::from_str::<Journal>(&contents)?;
    println!("Version: {}", journal.metadata.version);
    let num_entries = journal.entries.len();
    println!("Number of entries: {}", num_entries);
    let first_entry_date = journal.entries[0].creation_date.to_owned();
    println!("First entry date: {}", first_entry_date);
    let last_entry_date = journal.entries[num_entries - 1].creation_date.to_owned();
    println!("Last entry date: {}", last_entry_date);
    let first_entry_length = journal.entries[0].text.len();
    println!("First entry length: {}", first_entry_length);
    let last_entry_length = journal.entries[num_entries - 1].text.len();
    println!("Last entry length: {}", last_entry_length);
    // let first_entry_num_photos = match journal.entries[0].photos {
    //     Some(ref photos) => photos.len(),
    //     None => 0,
    // };
    // println!("First entry number of photos: {}", first_entry_num_photos);
    Ok(())
}
