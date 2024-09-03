use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Optgroup;
impl ElementName for Optgroup {
    fn name(&self) -> &'static str {
        "optgroup"
    }
}

impl<'a> ElementBuilder<'a> for Optgroup {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
