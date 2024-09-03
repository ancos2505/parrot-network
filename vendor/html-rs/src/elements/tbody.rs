use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Tbody;
impl ElementName for Tbody {
    fn name(&self) -> &'static str {
        "tbody"
    }
}

impl<'a> ElementBuilder<'a> for Tbody {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
