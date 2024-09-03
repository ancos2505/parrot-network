use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Progress;
impl ElementName for Progress {
    fn name(&self) -> &'static str {
        "progress"
    }
}

impl<'a> ElementBuilder<'a> for Progress {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
