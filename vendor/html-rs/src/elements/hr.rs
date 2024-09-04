use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Hr;
impl ElementName for Hr {
    fn name(&self) -> &'static str {
        "hr"
    }
}

impl<'a> ElementBuilder<'a> for Hr {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}