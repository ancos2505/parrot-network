use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Bdi;
impl ElementName for Bdi {
    fn name(&self) -> &'static str {
        "bdi"
    }
}

impl<'a> ElementBuilder<'a> for Bdi {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
