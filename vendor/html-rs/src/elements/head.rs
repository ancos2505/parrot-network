use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Head;
impl ElementName for Head {
    fn name(&self) -> &'static str {
        "head"
    }
}

impl<'a> ElementBuilder<'a> for Head {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
