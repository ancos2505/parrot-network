use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Section;
impl ElementName for Section {
    fn name(&self) -> &'static str {
        "section"
    }
}

impl<'a> ElementBuilder<'a> for Section {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
