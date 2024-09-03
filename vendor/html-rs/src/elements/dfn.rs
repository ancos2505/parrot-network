use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Dfn;
impl ElementName for Dfn {
    fn name(&self) -> &'static str {
        "dfn"
    }
}

impl<'a> ElementBuilder<'a> for Dfn {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
