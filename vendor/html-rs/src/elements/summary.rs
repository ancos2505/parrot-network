use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Summary;
impl ElementName for Summary {
    fn name(&self) -> &'static str {
        "summary"
    }
}

impl<'a> ElementBuilder<'a> for Summary {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
