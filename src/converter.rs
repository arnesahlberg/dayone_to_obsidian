use crate::types::Journal;
use std::{fs::File, io::Read, io::Write, error::Error};
use chrono::DateTime;
use std::path::Path;


pub fn convert_to_obsidian (
        input_folder : &str, 
        output_folder : &str,
        use_icons : bool, 
        tag_prefix : &str,
        always_tag : bool
    ) -> Result<(), Box<dyn Error>> {
    let filepath = Path::new(input_folder).join("Journal.json");
    let mut file = 
        match File::open(&filepath) {
            Ok(file) => file,
            Err(_) => {
                println!("File not found: {}", filepath.display());
                return Err(format!("File not found: {}", filepath.display()).into());
            }
        };
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let journal = serde_json::from_str::<Journal>(&contents)?;
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
                    location
                },
                None => String::new()
            }

        };



        let date_header = time_zone_date.format( "%A, %d %B %Y at %-I:%M %p").to_string();
        // page header
        new_entry.push_str(
          format!("# {}{}\n\n",
            date_icons,
            &date_header
          ).as_str()
        );

    

 

        // fix body text
        let mut text = 
            match entry.text{
                Some(ref text) => {
                    text
                    .replace("\\", "")
                    .replace("\u{2028}", "\n") // line separator
                },
                None => String::new(),
                
            };
        // don't add to new entry yet, but do after the photos part because we may want to put a photo at the top of the entry

        let year_folder =  Path::new(output_folder).join(time_zone_date.format("%Y").to_string());
        let month_folder = year_folder.join(time_zone_date.format("%m").to_string());
        // create monthfolder if not exists
        std::fs::create_dir_all(&month_folder)?;
        
        // put photos in the correct folder
        match entry.photos {
            Some(ref photos) => {
                for photo in photos {
                    let file_name = format!("{}.{}", photo.md5, photo.photo_type) ;
                    let new_file_name = format!("{}.{}", photo.identifier, photo.photo_type) ; 
                    let temp_path = { // older version had identifier instead of md5 as filename (and just a single file per entry)
                        let p = Path::new(input_folder).join("photos").join(&file_name);
                        if p.exists() {
                            p
                        } else {
                            Path::new(input_folder).join("photos").join( format!("{}.{}", photo.identifier, photo.photo_type))
                        }
                    } ;
                    // new file_name variable based on which path existed
                    let file_name = temp_path.file_name().unwrap().to_str().unwrap();


                    // check if file exists, if not skip
                    if !temp_path.exists() {
                        println!("Warning in entry from '{}': Cannot find file {}.", date_header, temp_path.to_str().unwrap());
                        text = text.replace(&format!("![](dayone-moment://{})", &file_name.replace(&format!(".{}", photo.photo_type), "")), "");
                        continue;
                    }
                    

                    let file_path = temp_path.to_str().unwrap();
                    
                    let temp_path = month_folder.join("photos").join(&new_file_name);
                    let new_file_path = temp_path.to_str().unwrap();
                    
                    // create folder if not exists
                    std::fs::create_dir_all(month_folder.join("photos"))?;
                    std::fs::copy(file_path, new_file_path)?;

                    // check if it's an entry from the older version where dayone-moment:// wasn't included in the file and
                    // there was just a single photo per entry at most. (Maybe I'm wrong about this?)
                    if text.contains("dayone-moment://") {
                        // replace part of the entry text to link to the file
                        let old_link = format!("![](dayone-moment://{})", photo.identifier) ; 
                        let photo_link = format!("![](photos/{})", new_file_name);
                        text = text.replace(&old_link, &photo_link);
                    }
                    else {
                        // add a link to the photo at the end of the entry
                        new_entry.push_str(&format!("\n\n![]({})", new_file_name));
                    }
                }
            },
            None => (),
        };

        // now the same with videos
        match entry.videos {
            Some(videos) => {
                for video in videos {
                    let file_name = format!("{}.{}", video.md5, video.video_type    ) ;
                    let new_file_name = format!("{}.{}", video.identifier, video.video_type) ; // all files have .mp4 extension afaik
                    let file_path = Path::new(input_folder).join("videos").join(&file_name);
                    // check if file exists, if not skip
                    if !file_path.exists() {
                        println!("Warning in entry from '{}': Cannot find file {}.", date_header, file_path.to_str().unwrap());
                        continue;
                    }
                    let file_path = file_path.to_str().unwrap(); // should never fail if here
                    
                    let temp_path = month_folder.join("videos").join(&new_file_name);
                    let new_file_path = temp_path.to_str().unwrap();

                    // create folder if not exists
                    std::fs::create_dir_all(month_folder.join("videos"))?;
                    std::fs::copy(file_path, new_file_path)?;

                    // replace in text
                    let old_link = format!("![](dayone-moment:/video/{})", video.identifier) ;
                    let video_link = format!("![](videos/{})", new_file_name);
                    text = text.replace(&old_link, &video_link);
                }
            }
            None => (),
        };

        // now add the body text
        new_entry.push_str(&text);


        let mut frontmatter = format!("\n\n---\n- created: {}", time_zone_date.format("%Y-%m-%d %H:%M:%S").to_string());
        match entry.location {
            Some(ref loc) => {
                let coordinates = format!("[{},{}]", loc.latitude, loc.longitude);
                frontmatter.push_str(&format!("\n- location: {} {}", location, coordinates));
            },
            None => (),
        };
        frontmatter.push_str("\n---\n");
        new_entry.push_str(&frontmatter);



        let tags = {
            let mut tags_string = String::new();
            let all_tags =
                match entry.tags {
                    Some(ref tags) => {
                        let mut tags_string = String::new();
                        for tag in tags {
                            let modified_tag = format!(" {}/{}", tag_prefix, tag.replace(" ", "-").replace("---", "-"));
                            tags_string.push_str(&modified_tag);
                        };
                        tags_string
                    },
                    None => String::new(),
                };
            tags_string.push_str(&all_tags);
            match entry.starred {
                true => {
                    tags_string.push_str(" starred");
                },
                false => (),
            };
            if tags_string.len() > 0 {
                format!("- Tags:\n{}", tags_string)
            }
            else if always_tag {
                format!("- Tags:\n{}", tag_prefix)
            }
            else {
                tags_string
            }
        } ;

        new_entry.push_str(&format!("\n\n{}", tags));
            

        // write the entry to the output folder
        let file_name = format!("{}.md", time_zone_date.format("%Y-%m-%d-%H-%M-%S"));
        let file_path = month_folder.join(file_name);
        let file_path = 
            match file_path.to_str() {
                Some(path) => path,
                None => "",
            };

        let mut file = File::create(file_path)?;
        file.write_all(new_entry.as_bytes())?;

    };
    println!("Done processing entries.");


    Ok(())
}
