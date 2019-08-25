use dirs;
use glob::glob;
use lz4::block;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::str;

fn main() {
    let home = dirs::home_dir().unwrap_or_else(|| panic!("Couldn't find home directory"));
    let path: PathBuf = [
        home.to_str().unwrap(),
        "Library",
        "Application Support",
        "Firefox",
        "Profiles",
        "*default*",
        "sessionstore-backups",
        "recovery.jsonlz4"
    ].iter().collect();

    for result in glob(path.to_str().unwrap()).unwrap() {
        let item = result.unwrap();
        match decompressed_contents(item) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e)
        }
    }
}

fn decompressed_contents(item: PathBuf) -> io::Result<String> {
    let s = decompress(&Path::new(&item))?;
    Ok(convert_to_string(&s)?.to_string())
}

// Re-wrap the `Utf8Error` in `str::from_utf8` in an `io::Error` so we can always return an
// `io::Result` in `decompressed_contents`
fn convert_to_string<'a>(blocks: &'a Vec<u8>) -> io::Result<&'a str> {
    str::from_utf8(&blocks[..]).map_err(|e| io::Error::new(ErrorKind::Other, e))
}

fn decompress(source: &Path) -> io::Result<Vec<u8>> {
    let mut input_file = File::open(source)?;
    let mut input_buffer = Vec::new();
    input_file.read_to_end(&mut input_buffer)?;
    // Skip the first 8 bytes: "mozLz40\0"
    block::decompress(&input_buffer[8..], None)
}
