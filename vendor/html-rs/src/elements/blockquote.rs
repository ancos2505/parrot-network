use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Blockquote;
impl ElementName for Blockquote {
    fn name(&self) -> &'static str {
        "blockquote"
    }
}

impl<'a> ElementBuilder<'a> for Blockquote {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
