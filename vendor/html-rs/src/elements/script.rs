use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Script;
impl ElementName for Script {
    fn name(&self) -> &'static str {
        "script"
    }
}

impl<'a> ElementBuilder<'a> for Script {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
