use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Dd;
impl ElementName for Dd {
    fn name(&self) -> &'static str {
        "dd"
    }
}

impl<'a> ElementBuilder<'a> for Dd {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
