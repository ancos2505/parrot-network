use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Template;
impl ElementName for Template {
    fn name(&self) -> &'static str {
        "template"
    }
}

impl<'a> ElementBuilder<'a> for Template {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
