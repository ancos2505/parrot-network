use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Meta;
impl ElementName for Meta {
    fn name(&self) -> &'static str {
        "meta"
    }
}

impl<'a> ElementBuilder<'a> for Meta {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
