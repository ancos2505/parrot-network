use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Sub;
impl ElementName for Sub {
    fn name(&self) -> &'static str {
        "sub"
    }
}

impl<'a> ElementBuilder<'a> for Sub {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
