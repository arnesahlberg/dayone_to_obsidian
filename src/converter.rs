use crate::types::Journal;
use std::{fs::File, io::Read, io::Write, error::Error};
use chrono::DateTime;

pub fn convert_to_obsidian (journal : Journal, output_folder : String, use_icons : bool) -> Result<(), Box<dyn Error>> {
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
        println!("Processing entry: {} (local time {})", created_date, time_zone_date);

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
        new_entry.push_str(&format!("\n\n{}", entry.text));

        println!("{}", new_entry);
    };
    println!("Done processing entries.");


    Ok(())
}
