use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Table;
impl ElementName for Table {
    fn name(&self) -> &'static str {
        "table"
    }
}

impl<'a> ElementBuilder<'a> for Table {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
