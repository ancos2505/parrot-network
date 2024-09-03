use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Hgroup;
impl ElementName for Hgroup {
    fn name(&self) -> &'static str {
        "hgroup"
    }
}

impl<'a> ElementBuilder<'a> for Hgroup {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
