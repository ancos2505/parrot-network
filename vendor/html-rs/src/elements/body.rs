use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Body;
impl ElementName for Body {
    fn name(&self) -> &'static str {
        "body"
    }
}

impl<'a> ElementBuilder<'a> for Body {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
