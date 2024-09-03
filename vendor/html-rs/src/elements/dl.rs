use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Dl;
impl ElementName for Dl {
    fn name(&self) -> &'static str {
        "dl"
    }
}

impl<'a> ElementBuilder<'a> for Dl {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
