use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Col;
impl ElementName for Col {
    fn name(&self) -> &'static str {
        "col"
    }
}

impl<'a> ElementBuilder<'a> for Col {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
