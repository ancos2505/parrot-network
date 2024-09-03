use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Br;
impl ElementName for Br {
    fn name(&self) -> &'static str {
        "br"
    }
}

impl<'a> ElementBuilder<'a> for Br {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
