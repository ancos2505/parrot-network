use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Video;
impl ElementName for Video {
    fn name(&self) -> &'static str {
        "video"
    }
}

impl<'a> ElementBuilder<'a> for Video {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
