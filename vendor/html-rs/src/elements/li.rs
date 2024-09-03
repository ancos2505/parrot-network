use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Li;
impl ElementName for Li {
    fn name(&self) -> &'static str {
        "li"
    }
}

impl<'a> ElementBuilder<'a> for Li {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
