use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Label;
impl ElementName for Label {
    fn name(&self) -> &'static str {
        "label"
    }
}

impl<'a> ElementBuilder<'a> for Label {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
