use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Main;
impl ElementName for Main {
    fn name(&self) -> &'static str {
        "main"
    }
}

impl<'a> ElementBuilder<'a> for Main {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
