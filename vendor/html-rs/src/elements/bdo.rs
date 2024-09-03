use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Bdo;
impl ElementName for Bdo {
    fn name(&self) -> &'static str {
        "bdo"
    }
}

impl<'a> ElementBuilder<'a> for Bdo {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
