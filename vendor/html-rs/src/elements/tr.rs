use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Tr;
impl ElementName for Tr {
    fn name(&self) -> &'static str {
        "tr"
    }
}

impl<'a> ElementBuilder<'a> for Tr {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
