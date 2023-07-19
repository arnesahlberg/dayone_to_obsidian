# dayone_to_obsidian

This is a script to convert Day One JSON exports to Obsidian compatible markdown files. It is currently a work in progress, but it is functional, though without guarantees.

## Usage

1. Export your Day One journal as JSON files.
2. Clone this repo.
3. Build with `cargo build --release`
4. Run with `./target/release/dayone_to_obsidian --input-folder <path to folder with Day One export containing Journal.json> --output-folder <desired output folder>` 
5. Import the output folder into Obsidian by moving it into your Obsidian folder.

## Additional Settings

- Use the flag `--icons` if you use the Icons Plugin for Obsidian.
- Use the setting `--tag-prefix <tag-prefix>` to use a custom tag prefix for your day one tags. The default is `#journal`.
- Use the flag `--no-tag-if-empty` to not add a tag if the entry has no tags. Otherwise every Day One entry without a tag will be tagged with the tag-prefix.


## TODO

- Add importing of videos and PDFs.



