use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Input;
impl ElementName for Input {
    fn name(&self) -> &'static str {
        "input"
    }
}

impl<'a> ElementBuilder<'a> for Input {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
