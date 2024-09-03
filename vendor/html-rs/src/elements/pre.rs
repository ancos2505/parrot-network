use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Pre;
impl ElementName for Pre {
    fn name(&self) -> &'static str {
        "pre"
    }
}

impl<'a> ElementBuilder<'a> for Pre {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
