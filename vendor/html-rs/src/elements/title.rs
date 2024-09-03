use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Title;
impl<'a> Title {
    pub fn text<S: AsRef<str>>(text: S) -> HtmlElement<'a> {
        Self::builder().inner_text(text.as_ref())
    }
}

impl ElementName for Title {
    fn name(&self) -> &'static str {
        "title"
    }
}

impl<'a> ElementBuilder<'a> for Title {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
