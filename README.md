# hls_parser

A super simple parser that parses an HLS file. 

This parser is mostly based on the [HTTP Live Streaming Protocol Doc](https://tools.ietf.org/html/rfc8216) with some modifications for time constraints. It makes a blocking HTTP request to grab a text-based master playlist 
from an endpoint, parse it into a mostly valid HLS subset, sort the individual sets of tags, and print it out!

## Prereqs

To use this HLS parser, you must have [Rust](https://www.rust-lang.org/tools/install) installed.

## Usage

To run the parser, use `cargo run` in the hls_parser directory. This will build and run the binary.

There are also some tests! You can run those by using `cargo test` in the hls_parser directory. 

## Stretch Goals

I wanted to make a fancy CLI with different sorting options but learning how to write a parser was pretty tricky and ate a bunch of my time.

I would also like to have some better error handling so that when we read in an invalid piece of HLS we can give a clearer message about what's gone wrong where.

Also there are never enough tests. This would have been fun to use some [Property](https://github.com/AltSysrq/proptest) [Based](https://github.com/BurntSushi/quickcheck) testing here.
