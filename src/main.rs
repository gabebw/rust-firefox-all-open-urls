use dirs;
use glob::glob;
use lz4::block;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::str;

fn main() {
    let mut path = format!("{}", dirs::home_dir().unwrap().display());
    path.push_str("/Library/Application Support/Firefox/Profiles");
    path.push_str("/*default*/sessionstore-backups/recovery.jsonlz4");
    let path = "recovery.jsonlz4";

    for result in glob(&path).unwrap() {
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
    match str::from_utf8(&blocks[..]) {
        Ok(s) => Ok(s),
        Err(e) => Err(io::Error::new(ErrorKind::Other, e))
    }
}

fn decompress(source: &Path) -> io::Result<Vec<u8>> {
    let mut input_file = File::open(source)?;
    let mut input_buffer = Vec::new();
    input_file.seek(SeekFrom::Start(8))?;
    input_file.read_to_end(&mut input_buffer)?;
    block::decompress(&input_buffer[..], None)
}
