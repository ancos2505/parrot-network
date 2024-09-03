use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Audio;
impl ElementName for Audio {
    fn name(&self) -> &'static str {
        "audio"
    }
}

impl<'a> ElementBuilder<'a> for Audio {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
