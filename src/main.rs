use std::error::Error;
pub mod types;
pub mod converter;
use converter::convert_to_obsidian;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "converter")]
struct Options {
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
    #[structopt(long = "tag-prefix", short="t", default_value = "#journal")]
    tag_prefix: String,

    // if this flag is not used every entry will be tagged with the tag-prefix even if the original entry is not tagged
    #[structopt(long = "no-tag-if-empty")]
    no_tag_if_empty: bool,

}

fn main() -> Result<(), Box<dyn Error>> {
    let flags = Options::from_args();

    if flags.tag_prefix.ends_with("/") {
        return Err("A tag-prefix ending with '/' is implicitly added. Please remove the '/' from the tag-prefix.".into());
    }

    convert_to_obsidian(
        &flags.input_folder, 
        &flags.output_folder, 
        flags.icons, 
        &flags.tag_prefix, 
        !flags.no_tag_if_empty
    )?;

    Ok(())
}
