use std::{fs::File, io::Read};
use std::error::Error;
pub mod types;
use types::Journal;
pub mod converter;
use converter::convert_to_obsidian;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "converter")]

struct Flags {
    // Input folder
    #[structopt(short = "i", long = "input-folder")]
    input_folder: String,

    // Output folder
    #[structopt(short = "o", long = "output-folder")]
    output_folder: String,

    // if the Icons Plugin is used?
    #[structopt(long = "icons")]
    icons: bool,

    // tag prefix
    #[structopt(long = "tag-prefix", short="t", default_value = "#journal/")]
    tag_prefix: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let flags = Flags::from_args();

    convert_to_obsidian(&flags.input_folder, &flags.output_folder, flags.icons, &flags.tag_prefix)?;
    Ok(())
}
