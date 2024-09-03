use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Dialog;
impl ElementName for Dialog {
    fn name(&self) -> &'static str {
        "dialog"
    }
}

impl<'a> ElementBuilder<'a> for Dialog {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
