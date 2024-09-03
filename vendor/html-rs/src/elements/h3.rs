use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct H3;
impl ElementName for H3 {
    fn name(&self) -> &'static str {
        "h3"
    }
}

impl<'a> ElementBuilder<'a> for H3 {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
