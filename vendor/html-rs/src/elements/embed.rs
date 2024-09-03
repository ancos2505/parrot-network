use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Embed;
impl ElementName for Embed {
    fn name(&self) -> &'static str {
        "embed"
    }
}

impl<'a> ElementBuilder<'a> for Embed {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
