use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Address;
impl ElementName for Address {
    fn name(&self) -> &'static str {
        "address"
    }
}

impl<'a> ElementBuilder<'a> for Address {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
