use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct H1;
impl ElementName for H1 {
    fn name(&self) -> &'static str {
        "h1"
    }
}

impl<'a> ElementBuilder<'a> for H1 {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
