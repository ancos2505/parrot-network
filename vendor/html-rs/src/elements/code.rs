use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Code;
impl ElementName for Code {
    fn name(&self) -> &'static str {
        "code"
    }
}

impl<'a> ElementBuilder<'a> for Code {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
