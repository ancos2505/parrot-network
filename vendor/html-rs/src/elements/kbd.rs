use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Kbd;
impl ElementName for Kbd {
    fn name(&self) -> &'static str {
        "kbd"
    }
}

impl<'a> ElementBuilder<'a> for Kbd {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
