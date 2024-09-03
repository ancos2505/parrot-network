use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Thead;
impl ElementName for Thead {
    fn name(&self) -> &'static str {
        "thead"
    }
}

impl<'a> ElementBuilder<'a> for Thead {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
