use dirs;
use glob::glob;
use lz4::block;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path};
use std::result;
use std::str::{self, Utf8Error};

fn main() {
    let mut path = format!("{}", dirs::home_dir().unwrap().display());
    path.push_str("/Library/Application Support/Firefox/Profiles");
    path.push_str("/*default*/sessionstore-backups/recovery.jsonlz4");
    let path = "recovery.jsonlz4";
    for result in glob(&path).unwrap() {
        // result is Result<std::path::PathBuf, glob::GlobError>
        if let Ok(item) = result {
            match decompress(&Path::new(&item)) {
                Ok(s) => println!("{}", convert_to_string(&s).unwrap()),
                Err(e) => eprintln!("Error: {}", e)
            }
        } else {
            eprintln!("Glob error");
        }
    }
}

fn convert_to_string<'a>(blocks: &'a Vec<u8>) -> result::Result<&'a str, Utf8Error> {
    str::from_utf8(&blocks[..])
}

fn decompress(source: &Path) -> io::Result<Vec<u8>> {
    let mut input_file = File::open(source)?;
    let mut input_buffer = Vec::new();
    input_file.seek(SeekFrom::Start(8))?;
    input_file.read_to_end(&mut input_buffer)?;
    block::decompress(&input_buffer[..], None)
}
