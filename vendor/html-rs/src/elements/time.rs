use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Time;
impl ElementName for Time {
    fn name(&self) -> &'static str {
        "time"
    }
}

impl<'a> ElementBuilder<'a> for Time {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
