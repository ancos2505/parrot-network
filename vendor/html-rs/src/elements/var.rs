use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Var;
impl ElementName for Var {
    fn name(&self) -> &'static str {
        "var"
    }
}

impl<'a> ElementBuilder<'a> for Var {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
