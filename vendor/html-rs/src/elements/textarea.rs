use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Textarea;
impl ElementName for Textarea {
    fn name(&self) -> &'static str {
        "textarea"
    }
}

impl<'a> ElementBuilder<'a> for Textarea {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
