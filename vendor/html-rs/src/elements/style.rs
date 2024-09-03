use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Style;
impl ElementName for Style {
    fn name(&self) -> &'static str {
        "style"
    }
}

impl<'a> ElementBuilder<'a> for Style {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
