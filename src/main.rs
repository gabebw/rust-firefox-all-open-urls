use dirs;
use glob::glob;
use lz4::block;
use serde::{Deserialize};
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::str;

#[derive(Deserialize)]
struct TopLevel {
    windows: Vec<Window>,
}

#[derive(Deserialize)]
struct Window {
    tabs: Vec<Tab>,
}

#[derive(Deserialize)]
struct Tab {
    entries: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    url: String,
}

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
        run(item).unwrap_or_else(|e| eprintln!("{}", e));
    }
}

fn run(item: PathBuf) -> io::Result<()> {
    let json = decompressed_contents(item)?;
    let urls = parse_json(&json)?;
    for url in urls {
        println!("{}", url);
    }
    Ok(())
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

fn is_good_url(entry: Entry) -> Option<String> {
    if &entry.url[..6] == "about:" {
        None
    } else {
        Some(entry.url.to_string())
    }
}

fn parse_json(json: &str) -> serde_json::Result<Vec<String>> {
    let v: TopLevel = serde_json::from_str(json)?;
    Ok(v.windows.into_iter().flat_map(|window|
        window.tabs.into_iter().flat_map(|tab|
            tab.entries.into_iter().filter_map(is_good_url)
        )
    ).collect())
}
