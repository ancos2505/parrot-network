use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Link;
impl ElementName for Link {
    fn name(&self) -> &'static str {
        "link"
    }
}

impl<'a> ElementBuilder<'a> for Link {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
