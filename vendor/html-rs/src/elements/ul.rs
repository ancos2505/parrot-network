use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Ul;
impl ElementName for Ul {
    fn name(&self) -> &'static str {
        "ul"
    }
}

impl<'a> ElementBuilder<'a> for Ul {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
