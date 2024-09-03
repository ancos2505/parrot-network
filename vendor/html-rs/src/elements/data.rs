use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Data;
impl ElementName for Data {
    fn name(&self) -> &'static str {
        "data"
    }
}

impl<'a> ElementBuilder<'a> for Data {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
