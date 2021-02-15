#[derive(Debug, PartialEq)]
pub(crate) enum Tag {
    TagWithAttributes(TagWithAttributes),
    TagWithURI(TagWithURI),
    BasicTag(BasicTag),
}

#[derive(Debug, PartialEq)]
pub(crate) struct TagWithURI {
    pub(crate) name: String,
    pub(crate) attributes: Vec<(String, String)>,
    pub(crate) uri: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TagWithAttributes {
    pub(crate) name: String,
    pub(crate) attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct BasicTag {
    pub(crate) name: String,
}
