use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Iframe;
impl ElementName for Iframe {
    fn name(&self) -> &'static str {
        "iframe"
    }
}

impl<'a> ElementBuilder<'a> for Iframe {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}