use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Rp;
impl ElementName for Rp {
    fn name(&self) -> &'static str {
        "rp"
    }
}

impl<'a> ElementBuilder<'a> for Rp {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
