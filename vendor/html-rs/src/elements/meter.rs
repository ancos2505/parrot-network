use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Meter;
impl ElementName for Meter {
    fn name(&self) -> &'static str {
        "meter"
    }
}

impl<'a> ElementBuilder<'a> for Meter {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
