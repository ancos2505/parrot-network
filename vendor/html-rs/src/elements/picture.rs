use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Picture;
impl ElementName for Picture {
    fn name(&self) -> &'static str {
        "picture"
    }
}

impl<'a> ElementBuilder<'a> for Picture {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
