use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Ruby;
impl ElementName for Ruby {
    fn name(&self) -> &'static str {
        "ruby"
    }
}

impl<'a> ElementBuilder<'a> for Ruby {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
