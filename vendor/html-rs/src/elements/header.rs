use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Header;
impl ElementName for Header {
    fn name(&self) -> &'static str {
        "header"
    }
}

impl<'a> ElementBuilder<'a> for Header {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
