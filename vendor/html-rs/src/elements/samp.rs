use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Samp;
impl ElementName for Samp {
    fn name(&self) -> &'static str {
        "samp"
    }
}

impl<'a> ElementBuilder<'a> for Samp {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
