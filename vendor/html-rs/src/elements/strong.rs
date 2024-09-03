use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Strong;
impl ElementName for Strong {
    fn name(&self) -> &'static str {
        "strong"
    }
}

impl<'a> ElementBuilder<'a> for Strong {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
