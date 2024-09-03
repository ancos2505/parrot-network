use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Wbr;
impl ElementName for Wbr {
    fn name(&self) -> &'static str {
        "wbr"
    }
}

impl<'a> ElementBuilder<'a> for Wbr {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
