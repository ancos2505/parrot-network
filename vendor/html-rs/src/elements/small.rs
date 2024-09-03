use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Small;
impl ElementName for Small {
    fn name(&self) -> &'static str {
        "small"
    }
}

impl<'a> ElementBuilder<'a> for Small {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
