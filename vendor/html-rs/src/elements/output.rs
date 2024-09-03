use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Output;
impl ElementName for Output {
    fn name(&self) -> &'static str {
        "output"
    }
}

impl<'a> ElementBuilder<'a> for Output {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
