use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Span;
impl ElementName for Span {
    fn name(&self) -> &'static str {
        "span"
    }
}

impl<'a> ElementBuilder<'a> for Span {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
