use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Ins;
impl ElementName for Ins {
    fn name(&self) -> &'static str {
        "ins"
    }
}

impl<'a> ElementBuilder<'a> for Ins {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
