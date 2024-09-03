use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Canvas;
impl ElementName for Canvas {
    fn name(&self) -> &'static str {
        "canvas"
    }
}

impl<'a> ElementBuilder<'a> for Canvas {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
