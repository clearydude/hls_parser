#[cfg(test)]
mod tests;

use crate::errors::{Error, Result};

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};

use crate::models::{BasicTag, Tag, TagWithAttributes, TagWithURI};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated};

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(e: nom::Err<nom::error::Error<&str>>) -> Self {
        Error::ParseError(e.to_string())
    }
}

pub(crate) struct HLSParser {}

impl HLSParser {
    /// Takes a string representing an HLS file and parses it into a list of Tags
    pub(crate) fn parse(&self, hls_str: &str) -> Result<Vec<Tag>> {
        let (_, res) = parse_master_playlist(hls_str)?;

        Ok(res)
    }
}

fn parse_value(attr_str: &str) -> nom::IResult<&str, String> {
    map(is_not(",\n"), |attr: &str| attr.to_string())(attr_str)
}

fn parse_quoted_value(attr_str: &str) -> nom::IResult<&str, String> {
    map(
        delimited(tag("\""), is_not("\""), tag("\"")),
        |attr: &str| attr.to_string(),
    )(attr_str)
}

fn parse_attribute(attr_str: &str) -> nom::IResult<&str, (String, String)> {
    map(
        separated_pair(
            take_until("="),
            tag("="),
            alt((parse_quoted_value, parse_value)),
        ),
        |(key, value)| (key.to_string(), value.to_string()),
    )(attr_str)
}

fn parse_attributes(attrs_str: &str) -> nom::IResult<&str, Vec<(String, String)>> {
    separated_list1(tag(","), parse_attribute)(attrs_str)
}

fn parse_tag_name(tag_str: &str) -> nom::IResult<&str, String> {
    map(preceded(tag("#"), is_not(":\n")), |name: &str| {
        name.to_string()
    })(tag_str)
}

fn parse_tag_and_attributes(tag_str: &str) -> nom::IResult<&str, (String, Vec<(String, String)>)> {
    separated_pair(parse_tag_name, tag(":"), parse_attributes)(tag_str)
}

fn parse_variant_stream(variant_stream_str: &str) -> nom::IResult<&str, Tag> {
    map(
        separated_pair(parse_tag_and_attributes, tag("\n"), parse_value),
        |((name, attributes), uri)| Tag::TagWithURI(TagWithURI::new(name, attributes, uri)),
    )(variant_stream_str)
}

fn parse_tag_with_attributes(tag_w_attributes_str: &str) -> nom::IResult<&str, Tag> {
    map(parse_tag_and_attributes, |(name, attributes)| {
        Tag::TagWithAttributes(TagWithAttributes::new(name, attributes))
    })(tag_w_attributes_str)
}

fn parse_simple_tag(simple_tag_str: &str) -> nom::IResult<&str, Tag> {
    map(parse_tag_name, |name| Tag::BasicTag(BasicTag::new(name)))(simple_tag_str)
}

fn parse_master_playlist(playlist_str: &str) -> nom::IResult<&str, Vec<Tag>> {
    many1(preceded(
        multispace0,
        terminated(
            alt((
                parse_variant_stream,
                parse_tag_with_attributes,
                parse_simple_tag,
            )),
            multispace0,
        ),
    ))(playlist_str)
}
