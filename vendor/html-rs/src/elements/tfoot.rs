use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Tfoot;
impl ElementName for Tfoot {
    fn name(&self) -> &'static str {
        "tfoot"
    }
}

impl<'a> ElementBuilder<'a> for Tfoot {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
