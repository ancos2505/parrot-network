use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Del;
impl ElementName for Del {
    fn name(&self) -> &'static str {
        "del"
    }
}

impl<'a> ElementBuilder<'a> for Del {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
