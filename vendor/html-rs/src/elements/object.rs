use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Object;
impl ElementName for Object {
    fn name(&self) -> &'static str {
        "object"
    }
}

impl<'a> ElementBuilder<'a> for Object {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
