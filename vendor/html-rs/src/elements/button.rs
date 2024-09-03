use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Button;
impl ElementName for Button {
    fn name(&self) -> &'static str {
        "button"
    }
}

impl<'a> ElementBuilder<'a> for Button {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
