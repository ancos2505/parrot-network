use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct H6;
impl ElementName for H6 {
    fn name(&self) -> &'static str {
        "h6"
    }
}

impl<'a> ElementBuilder<'a> for H6 {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
