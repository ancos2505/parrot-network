use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct H2;
impl ElementName for H2 {
    fn name(&self) -> &'static str {
        "h2"
    }
}

impl<'a> ElementBuilder<'a> for H2 {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
