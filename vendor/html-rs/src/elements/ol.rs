use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Ol;
impl ElementName for Ol {
    fn name(&self) -> &'static str {
        "ol"
    }
}

impl<'a> ElementBuilder<'a> for Ol {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
