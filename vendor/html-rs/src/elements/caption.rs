use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Caption;
impl ElementName for Caption {
    fn name(&self) -> &'static str {
        "caption"
    }
}

impl<'a> ElementBuilder<'a> for Caption {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
