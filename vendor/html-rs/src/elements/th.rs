use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Th;
impl ElementName for Th {
    fn name(&self) -> &'static str {
        "th"
    }
}

impl<'a> ElementBuilder<'a> for Th {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
