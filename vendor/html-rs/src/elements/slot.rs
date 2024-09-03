use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Slot;
impl ElementName for Slot {
    fn name(&self) -> &'static str {
        "slot"
    }
}

impl<'a> ElementBuilder<'a> for Slot {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
