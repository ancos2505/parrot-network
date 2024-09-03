use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Colgroup;
impl ElementName for Colgroup {
    fn name(&self) -> &'static str {
        "colgroup"
    }
}

impl<'a> ElementBuilder<'a> for Colgroup {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
