use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Base;
impl ElementName for Base {
    fn name(&self) -> &'static str {
        "base"
    }
}

impl<'a> ElementBuilder<'a> for Base {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
