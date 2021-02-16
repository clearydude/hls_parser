/// The attribute properties in this file are largely written to track the specification listed
/// in the [HTTP Live Streaming RFC](https://tools.ietf.org/html/rfc8216). Some modifications have
/// been made where practical and for timing purposes.
/// In particular, the only attributes these models will map to are the set of attributes we know
/// exist in the provided m3u8 file example.
/// The only tags that are represented here are the tags that we specifically deal with in the
/// example playlist.
/// VIDEO-RANGE could only be found in a [draft](https://tools.ietf.org/html/draft-pantos-hls-rfc8216bis-00)
/// so that diverges from the original RFC slightly.
mod conversions;
#[cfg(test)]
mod tests;

use crate::errors::Error;

use std::num::ParseIntError;

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::HLSFormat(err.to_string())
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Default)]
/// The optimal pixel resolution in width and height.
struct Resolution {
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
/// Related to the encoding of the video.
enum VideoRange {
    PQ,
    SDR,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
/// A variant stream is a set of renditions that can be combined to play a media presentation.
pub(crate) struct VariantStream {
    /// A media playlist that carries a rendition of this variant stream.
    uri: String,
    /// The peak segment bit rate in bits per second.
    bandwidth: u32,
    /// The average segment bit rate in bits per second.
    average_bandwidth: u32,
    /// A comma seperated list of formats.
    codecs: String,
    /// The optimal pixel resolution to display this video.
    resolution: Resolution,
    /// Either PQ or SDR, this value is related to the encoding.
    video_range: VideoRange,
    // the trait `Ord` is not implemented for `f32`
    // and in this case it's not useful to sort on since they are all the same
    // so just using a String
    /// The maximum frame rate for all videos in this stream, rounded to three decimal places.
    frame_rate: String,
    /// This value, if specified, must match the GROUP-ID value for a media tag (`TYPE=AUDIO`).
    /// It indicates the set of audio renditions to use when playing this presentation.
    audio: String,
    /// This value, if specified, must match the GROUP-ID value for a media tag (`TYPE=CLOSED-CAPTIONS`).
    /// It specifies which captions can be used to play this presentation.
    closed_captions: String,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
enum MediaType {
    Audio,
    Video,
    Subtitles,
    ClosedCaptions,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
/// A media tag is used to relate media playlists that contain alternative Renditions of the same
/// content.
pub(crate) struct MediaTag {
    /// The type of media specified by this media tag.
    media_type: MediaType,
    /// Associates this media item with its group.
    group_id: String,
    /// A human readable description.
    name: String,
    /// The primary language used in this rendition.
    language: String,
    // I wanted to get these next two values, default and autoselect
    // into bool but ran into compiler issues
    // and apparently rust doesn't automagically convert YES/NO to bools.
    /// The client should play this rendition by default.
    default: String,
    /// The client *may* play this rendition by default. Matches current playblack environment.
    autoselect: String,
    /// A backslash separated list of parameters. These parameters vary based on the TYPE of media.
    channels: String,
    /// Uri that identifies the media playlist file.
    /// If the media_type is ClosedCaptions this must not be present.
    uri: String,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
/// A media playlist containing the I-frames of a multimedia presentation.
pub(crate) struct IFrame {
    /// The average segment bit rate in bits per second.
    bandwidth: u32,
    /// A comma seperated list of formats.
    codecs: String,
    /// The optimal pixel resolution to display this I-frame.
    resolution: Resolution,
    /// Either PQ or SDR, this value is related to the encoding.
    video_range: VideoRange,
    /// The uri identifying this I-frame's media playlist file.
    uri: String,
}

#[derive(Debug, PartialEq, Default)]
/// A set of variant streams, each of which describes a different version of the same content.
pub(crate) struct MasterPlaylist {
    variant_streams: Vec<VariantStream>,
    media_tags: Vec<MediaTag>,
    i_frames: Vec<IFrame>,
    basic_tags: Vec<String>,
}
