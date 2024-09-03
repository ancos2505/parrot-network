use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Search;
impl ElementName for Search {
    fn name(&self) -> &'static str {
        "search"
    }
}

impl<'a> ElementBuilder<'a> for Search {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
