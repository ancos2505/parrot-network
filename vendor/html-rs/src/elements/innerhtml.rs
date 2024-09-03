use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Innerhtml;
impl ElementName for Innerhtml {
    fn name(&self) -> &'static str {
        "innerhtml"
    }
}

impl<'a> ElementBuilder<'a> for Innerhtml {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
