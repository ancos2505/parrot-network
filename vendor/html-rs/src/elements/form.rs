use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Form;
impl ElementName for Form {
    fn name(&self) -> &'static str {
        "form"
    }
}

impl<'a> ElementBuilder<'a> for Form {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
