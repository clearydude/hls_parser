mod nom_parser;

use crate::errors::{Error, Result};
// use nom_parser::parse_master_playlist;
use std::collections::HashMap;

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(e: nom::Err<nom::error::Error<&str>>) -> Self {
        Error::Parse(e.to_string())
    }
}

pub(crate) struct HLSParser {}

impl HLSParser {
    /// Takes a string representing an HLS file and parses it into a list of Tags
    pub(crate) fn parse(
        &self,
        _hls_str: &str,
    ) -> Result<Vec<(String, Option<HashMap<String, String>>)>> {
        // let (_, res) = parse_master_playlist(hls_str)?;
        //
        // Ok(res)
        unimplemented!()
    }
}
