use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Mark;
impl ElementName for Mark {
    fn name(&self) -> &'static str {
        "mark"
    }
}

impl<'a> ElementBuilder<'a> for Mark {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}