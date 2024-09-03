use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Details;
impl ElementName for Details {
    fn name(&self) -> &'static str {
        "details"
    }
}

impl<'a> ElementBuilder<'a> for Details {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
