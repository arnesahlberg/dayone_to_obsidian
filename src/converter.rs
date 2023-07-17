use crate::types::Journal;
use std::{fs::File, io::Read, io::Write, error::Error};
use chrono::DateTime;
use std::path::Path;


pub fn convert_to_obsidian (input_folder : &str, output_folder : &str, use_icons : bool) -> Result<(), Box<dyn Error>> {
    let filepath = Path::new(input_folder).join("Journal.json");
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();

    let tag_prefix = "#journal/";

    file.read_to_string(&mut contents)?;
    let journal = serde_json::from_str::<Journal>(&contents)?;
    println!("Version: {}", journal.metadata.version);
    std::fs::create_dir_all(output_folder)?;

    let date_icons = 
        match use_icons {
            true => {
                "`fas:CalendarAlt` ".to_string()
            },
            false => String::new(),
        };

    println!("Begin processing entries...");

    // itearate over journal.entries
    for entry in journal.entries {
        let mut new_entry = String::new();
        let created_date = DateTime::parse_from_rfc3339(&entry.creation_date)? ;
        let time_zone = entry.time_zone.parse::<chrono_tz::Tz>()?;
        let time_zone_date = created_date.with_timezone(&time_zone);

        let location = {
            match entry.location {
                Some(ref loc) => {
                    let mut location = String::new();
                    match loc.place_name {
                        Some(ref place_name) => {
                            location.push_str(&format!(", {}", place_name));
                        },
                        None => (),
                    };                    
                    match loc.locality_name {
                        Some(ref locality_name) => {
                            location.push_str(&format!(", {}", locality_name));
                        },
                        None => (),
                    };
                    match loc.administrative_area {
                        Some(ref administrative_area) => {
                            location.push_str(&format!(", {}", administrative_area));
                        },
                        None => (),
                    };
                    match loc.country {
                        Some(ref country) => {
                            location.push_str(&format!(", {}", country));
                        },
                        None => (),
                    };
                    if location.len() > 0 {
                        location.remove(0);
                        location.remove(0);
                    };
                    Some(location)
                },
                None => None
            }

        };

        let mut frontmatter = format!("---\n- created: {}", time_zone_date.format("%Y-%m-%d %H:%M:%S").to_string());
        match entry.location {
            Some(ref loc) => {
                let coordinates = format!("[{},{}]", loc.latitude, loc.longitude);
                frontmatter.push_str(&format!("\n- location: {}", coordinates));
            },
            None => (),
        };
        frontmatter.push_str("\n---");

        new_entry.push_str(&frontmatter);

        // page header
        new_entry.push_str(
          format!("\n\n# {}{}",
            date_icons,
            time_zone_date.format( "%A, %d %B %Y at %-I:%M %p").to_string()
          ).as_str()
        );

        // add body text
        let text = 
            match entry.text{
                Some(ref text) => {
                    text
                    .replace("\\", "")
                    .replace("\u{2028}", "\n") // line separator
                },
                None => String::new(),
                
            };
        new_entry.push_str(&format!("\n\n{}", text));

        let year_folder =  Path::new(output_folder).join(time_zone_date.format("%Y").to_string());
        let month_folder = year_folder.join(time_zone_date.format("%m").to_string());
        // create monthfolder if not exists
        std::fs::create_dir_all(&month_folder)?;
        
 
        
        // put photos in the correct folder
        match entry.photos {
            Some(ref photos) => {
                for photo in photos {
                    let file_name = format!("{}.jpeg", photo.md5) ;
                    let new_file_name = format!("{}.jpeg", photo.identifier) ;
                    let temp_path = { // older version had identifier instead of md5 as filename (and just a single file per entry)
                        let p = Path::new(input_folder).join("photos").join(&file_name);
                        if p.exists() {
                            p
                        } else {
                            Path::new(input_folder).join("photos").join( format!("{}.jpeg", photo.identifier))
                        }
                    } ;
                    // check if file exists, if not skip
                    if !temp_path.exists() {
                        println!("File {} does not exist", temp_path.to_str().unwrap());
                        continue;
                    }
                    
                    // new file_name variable based on which path existed
                    let file_name = temp_path.file_name().unwrap().to_str().unwrap();

                    let file_path = 
                        match temp_path.to_str() {
                            Some(path) => path,
                            None => "",
                        };
                    let temp_path = month_folder.join(&new_file_name);
                    let new_file_path =
                        match temp_path.to_str() {
                            Some(path) => path,
                            None => "",
                        };
                    
                    println!("Copying {} to {}", file_path, new_file_path);
                    std::fs::copy(file_path, new_file_path)?;

                    // replace part of the entry text to link to the file
                    let old_link = format!("![](dayone-moment://{})", &file_name.replace(".jpeg", "")) ;
                    let photo_link = format!("![]({})", new_file_name);
                    new_entry = new_entry.replace(&old_link, &photo_link);
                }
            },
            None => (),
        };

        let tags = 
            match entry.tags {
                Some(ref tags) => {
                    let mut tags_string = String::new();
                    for tag in tags {
                        tags_string.push_str(&format!("\n{}{}", tag_prefix, tag));
                    };
                    tags_string
                },
                None => String::new(),
            };

        // write the entry to the output folder
        let file_name = format!("{}.md", entry.uuid);
        println!("{}", new_entry);
    };
    println!("Done processing entries.");


    Ok(())
}
