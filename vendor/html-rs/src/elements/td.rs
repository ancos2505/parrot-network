use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Td;
impl ElementName for Td {
    fn name(&self) -> &'static str {
        "td"
    }
}

impl<'a> ElementBuilder<'a> for Td {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
