# Show currently-open Firefox URLs

This crate prints out the URLs of every tab currently open in Firefox, one per
line. If Firefox is not open, it doesn't print anything (and exits
successfully).

## Installation

    cargo install --git https://github.com/gabebw/rust-firefox-all-open-urls

## How it works

When Firefox is open (and only when it's open), it creates a file called
`recovery.jsonlz4`. As the name implies, it's [LZ4][wikipedia]-compressed JSON.
Firefox adds a little twist, though: it adds `mozLz40\0` to the beginning of the
file. So:

1. First, we remove those 8 bytes of `mozLz40\0`,
2. then decompress the LZ4 data,
3. then parse the JSON and get the URLs

[wikipedia]: https://en.wikipedia.org/wiki/LZ4_(compression_algorithm)
