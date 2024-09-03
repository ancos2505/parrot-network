use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Abbr;
impl ElementName for Abbr {
    fn name(&self) -> &'static str {
        "abbr"
    }
}

impl<'a> ElementBuilder<'a> for Abbr {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
