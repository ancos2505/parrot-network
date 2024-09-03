use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Em;
impl ElementName for Em {
    fn name(&self) -> &'static str {
        "em"
    }
}

impl<'a> ElementBuilder<'a> for Em {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
