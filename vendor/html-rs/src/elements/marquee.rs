use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Marquee;
impl ElementName for Marquee {
    fn name(&self) -> &'static str {
        "marquee"
    }
}

impl<'a> ElementBuilder<'a> for Marquee {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
