use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Figure;
impl ElementName for Figure {
    fn name(&self) -> &'static str {
        "figure"
    }
}

impl<'a> ElementBuilder<'a> for Figure {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
