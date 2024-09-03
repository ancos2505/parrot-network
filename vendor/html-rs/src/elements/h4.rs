use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct H4;
impl ElementName for H4 {
    fn name(&self) -> &'static str {
        "h4"
    }
}

impl<'a> ElementBuilder<'a> for H4 {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
