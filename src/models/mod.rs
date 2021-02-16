mod conversions;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::errors::{Error, Result};
use nom::lib::std::collections::hash_map::RandomState;
use std::num::ParseIntError;

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::HLSFormat(err.to_string())
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Default)]
struct Resolution {
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Default)]
pub(crate) struct VariantStream {
    uri: String,
    bandwidth: u32,
    average_bandwidth: u32,
    codecs: String,
    resolution: Resolution,
    // would normally use an enum here but this document never contains the other variant, SDR, and
    // Rust complains about the unused type.
    video_range: String,
    // the trait `Ord` is not implemented for `f32`
    // and in this case it's not useful to sort on since they are all the same
    // so just using a String
    frame_rate: String,
    audio: String,
    closed_captions: String,
}

#[derive(Debug, Default)]
struct MasterPlaylist {
    variant_streams: Vec<VariantStream>,
    basic_tags: Vec<String>,
}
