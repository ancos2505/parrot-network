use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct B;
impl ElementName for B {
    fn name(&self) -> &'static str {
        "b"
    }
}

impl<'a> ElementBuilder<'a> for B {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
