use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Noscript;
impl ElementName for Noscript {
    fn name(&self) -> &'static str {
        "noscript"
    }
}

impl<'a> ElementBuilder<'a> for Noscript {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
