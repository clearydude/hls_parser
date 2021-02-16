use crate::errors::{Error, Result};
use crate::models::{MasterPlaylist, Resolution, VariantStream};
use std::collections::HashMap;
use std::convert::TryFrom;

impl TryFrom<String> for Resolution {
    type Error = Error;

    fn try_from(res: String) -> Result<Self, Self::Error> {
        // A cheapie way to support resolutions that come through delimited by an uppercase 'X'
        let normalized_res = res.to_lowercase();
        let mut res_iter = normalized_res.splitn(2, 'x');
        let width = res_iter
            .next()
            .ok_or(Error::HLSFormat(
                "Could not find width in resolution".to_string(),
            ))?
            .parse()?;
        let height = res_iter
            .next()
            .ok_or(Error::HLSFormat(
                "Could not find height in resolution".to_string(),
            ))?
            .parse()?;

        Ok(Self { width, height })
    }
}

impl TryFrom<HashMap<String, String>> for VariantStream {
    type Error = Error;

    fn try_from(attributes: HashMap<String, String>) -> Result<Self> {
        Ok(Self {
            uri: attributes
                .get("URI")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .clone(),
            bandwidth: attributes
                .get("BANDWIDTH")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .parse()?,
            average_bandwidth: attributes
                .get("AVERAGE-BANDWIDTH")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .parse()?,
            codecs: attributes
                .get("CODECS")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .clone(),
            resolution: Resolution::try_from(
                attributes
                    .get("RESOLUTION")
                    .ok_or(Error::HLSFormat("URI".to_string()))?
                    .clone(),
            )?,
            video_range: attributes
                .get("VIDEO-RANGE")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .clone(),
            frame_rate: attributes
                .get("FRAME-RATE")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .clone(),
            audio: attributes
                .get("AUDIO")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .clone(),
            closed_captions: attributes
                .get("CLOSED-CAPTIONS")
                .ok_or(Error::HLSFormat("URI".to_string()))?
                .clone(),
        })
    }
}

impl TryFrom<Vec<(String, Option<HashMap<String, String>>)>> for MasterPlaylist {
    type Error = Error;

    fn try_from(tags: Vec<(String, Option<HashMap<String, String>>)>) -> Result<Self, Self::Error> {
        let mut variant_streams = vec![];
        let mut basic_tags = vec![];
        for (name, attributes) in tags {
            match name.as_str() {
                "EXT-X-STREAM-INF" => {
                    variant_streams.push(VariantStream::try_from(attributes.ok_or(
                        Error::HLSFormat("Variant stream must have attributes".to_string()),
                    )?)?)
                }
                _ => match attributes {
                    Some(_) => {
                        return Err(Error::HLSFormat(format!(
                            "Unknown tag with attributes found: {}",
                            name
                        )))
                    }
                    None => basic_tags.push(name),
                },
            }
        }

        Ok(Self {
            variant_streams,
            basic_tags,
        })
    }
}
