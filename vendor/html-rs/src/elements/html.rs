use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Html;
impl ElementName for Html {
    fn name(&self) -> &'static str {
        "html"
    }
}

impl<'a> ElementBuilder<'a> for Html {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
