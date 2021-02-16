use crate::errors::{Error, Result};
use crate::models::{IFrame, MasterPlaylist, MediaTag, Resolution, VariantStream};
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

impl TryFrom<HashMap<String, String>> for MediaTag {
    type Error = Error;

    fn try_from(attributes: HashMap<String, String>) -> Result<Self> {
        Ok(Self {
            media_type: attributes
                .get("TYPE")
                .ok_or(Error::HLSFormat("HLS missing TYPE".to_string()))?
                .clone(),
            group_id: attributes
                .get("GROUP-ID")
                .ok_or(Error::HLSFormat("HLS missing GROUP-ID".to_string()))?
                .clone(),
            name: attributes
                .get("NAME")
                .ok_or(Error::HLSFormat("HLS missing NAME".to_string()))?
                .clone(),
            language: attributes
                .get("LANGUAGE")
                .ok_or(Error::HLSFormat("HLS missing LANGUAGE".to_string()))?
                .clone(),
            default: attributes
                .get("DEFAULT")
                .ok_or(Error::HLSFormat("HLS missing DEFAULT".to_string()))?
                .clone(),
            autoselect: attributes
                .get("AUTOSELECT")
                .ok_or(Error::HLSFormat("HLS missing AUTOSELECT".to_string()))?
                .clone(),
            channels: attributes
                .get("CHANNELS")
                .ok_or(Error::HLSFormat("HLS missing CHANNELS".to_string()))?
                .clone(),
            uri: attributes
                .get("URI")
                .ok_or(Error::HLSFormat("HLS missing URI".to_string()))?
                .clone(),
        })
    }
}

impl TryFrom<HashMap<String, String>> for IFrame {
    type Error = Error;

    fn try_from(attributes: HashMap<String, String>) -> Result<Self> {
        Ok(Self {
            bandwidth: attributes
                .get("BANDWIDTH")
                .ok_or(Error::HLSFormat("HLS missing BANDWIDTH".to_string()))?
                .parse()?,
            codecs: attributes
                .get("CODECS")
                .ok_or(Error::HLSFormat("HLS missing CODECS".to_string()))?
                .clone(),
            resolution: Resolution::try_from(
                attributes
                    .get("RESOLUTION")
                    .ok_or(Error::HLSFormat("HLS missing RESOLUTION".to_string()))?
                    .clone(),
            )?,
            video_range: attributes
                .get("VIDEO-RANGE")
                .ok_or(Error::HLSFormat("HLS missing VIDEO-RANGE".to_string()))?
                .clone(),
            uri: attributes
                .get("URI")
                .ok_or(Error::HLSFormat("HLS missing URI".to_string()))?
                .clone(),
        })
    }
}

impl TryFrom<HashMap<String, String>> for VariantStream {
    type Error = Error;

    fn try_from(attributes: HashMap<String, String>) -> Result<Self> {
        Ok(Self {
            uri: attributes
                .get("URI")
                .ok_or(Error::HLSFormat("HLS missing URI".to_string()))?
                .clone(),
            bandwidth: attributes
                .get("BANDWIDTH")
                .ok_or(Error::HLSFormat("HLS missing BANDWIDTH".to_string()))?
                .parse()?,
            average_bandwidth: attributes
                .get("AVERAGE-BANDWIDTH")
                .ok_or(Error::HLSFormat(
                    "HLS missing AVERAGE-BANDWIDTH".to_string(),
                ))?
                .parse()?,
            codecs: attributes
                .get("CODECS")
                .ok_or(Error::HLSFormat("HLS missing CODECS".to_string()))?
                .clone(),
            resolution: Resolution::try_from(
                attributes
                    .get("RESOLUTION")
                    .ok_or(Error::HLSFormat("HLS missing RESOLUTION".to_string()))?
                    .clone(),
            )?,
            video_range: attributes
                .get("VIDEO-RANGE")
                .ok_or(Error::HLSFormat("HLS missing VIDEO-RANGE".to_string()))?
                .clone(),
            frame_rate: attributes
                .get("FRAME-RATE")
                .ok_or(Error::HLSFormat("HLS missing FRAME-RATE".to_string()))?
                .clone(),
            audio: attributes
                .get("AUDIO")
                .ok_or(Error::HLSFormat("HLS missing AUDIO".to_string()))?
                .clone(),
            closed_captions: attributes
                .get("CLOSED-CAPTIONS")
                .ok_or(Error::HLSFormat("HLS missing CLOSED-CAPTIONS".to_string()))?
                .clone(),
        })
    }
}

impl TryFrom<Vec<(String, HashMap<String, String>)>> for MasterPlaylist {
    type Error = Error;

    fn try_from(tags: Vec<(String, HashMap<String, String>)>) -> Result<Self, Self::Error> {
        let mut variant_streams = vec![];
        let mut basic_tags = vec![];
        let mut media_tags = vec![];
        let mut i_frames = vec![];

        for (name, attributes) in tags {
            match name.as_str() {
                "EXT-X-STREAM-INF" => variant_streams.push(VariantStream::try_from(attributes)?),
                "EXT-X-MEDIA" => media_tags.push(MediaTag::try_from(attributes)?),
                "EXT-X-I-FRAME-STREAM-INF" => i_frames.push(IFrame::try_from(attributes)?),
                _ => {
                    if attributes.len() > 0 {
                        return Err(Error::HLSFormat(format!(
                            "Unknown tag with attributes found: {}",
                            name
                        )));
                    }
                    basic_tags.push(name);
                }
            }
        }

        // Sort everything now while we've got mutable refs
        variant_streams.sort();
        i_frames.sort();
        media_tags.sort();
        basic_tags.sort();

        Ok(Self {
            variant_streams,
            basic_tags,
            i_frames,
            media_tags,
        })
    }
}
