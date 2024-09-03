use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Track;
impl ElementName for Track {
    fn name(&self) -> &'static str {
        "track"
    }
}

impl<'a> ElementBuilder<'a> for Track {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
