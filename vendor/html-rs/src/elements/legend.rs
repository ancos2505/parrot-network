use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Legend;
impl ElementName for Legend {
    fn name(&self) -> &'static str {
        "legend"
    }
}

impl<'a> ElementBuilder<'a> for Legend {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
