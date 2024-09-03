use crate::tags::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct TextContent;

impl<'a> TextContent {
    pub fn text<S: AsRef<str>>(text: S) -> HtmlElement<'a> {
        Self::builder().inner_text(text.as_ref())
    }
}

impl ElementName for TextContent {
    fn name(&self) -> &'static str {
        ""
    }
}

impl<'a> ElementBuilder<'a> for TextContent {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
