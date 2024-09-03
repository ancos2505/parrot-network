use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Source;
impl ElementName for Source {
    fn name(&self) -> &'static str {
        "source"
    }
}

impl<'a> ElementBuilder<'a> for Source {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
