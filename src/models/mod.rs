use derive_new::new;

#[derive(Debug, PartialEq, new)]
pub(crate) enum Tag {
    TagWithAttributes(TagWithAttributes),
    TagWithURI(TagWithURI),
    BasicTag(BasicTag),
}

#[derive(Debug, PartialEq, new)]
pub(crate) struct TagWithURI {
    name: String,
    attributes: Vec<(String, String)>,
    uri: String,
}

#[derive(Debug, PartialEq, new)]
pub(crate) struct TagWithAttributes {
    name: String,
    attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, new)]
pub(crate) struct BasicTag {
    name: String,
}
