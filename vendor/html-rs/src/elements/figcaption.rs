use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Figcaption;
impl ElementName for Figcaption {
    fn name(&self) -> &'static str {
        "figcaption"
    }
}

impl<'a> ElementBuilder<'a> for Figcaption {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
