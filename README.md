# hls_parser

A super simple parser that parses an HLS file. 

This parser is mostly based on the [HTTP Live Streaming Protocol Doc](https://tools.ietf.org/html/rfc8216) with some modifications for time constraints. It makes a blocking HTTP request to grab a text-based master playlist 
from an endpoint, parse it into a mostly valid HLS subset, sort the individual sets of tags, and print it out!

## PreRequisites

To use this HLS parser, you must have [Rust](https://www.rust-lang.org/tools/install) installed.

## Usage

To run the parser, simply use `cargo run` in the hls_parser directory.

## Stretch Goals

I wanted to make a fancy CLI with different sorting options but learning how to write a parser was pretty tricky and ate a bunch of my time.

I would also like to have some better error handling so that when we read in an invalid piece of HLS we can give a clearer message about what's gone wrong where.


