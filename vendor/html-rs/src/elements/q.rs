use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Q;
impl ElementName for Q {
    fn name(&self) -> &'static str {
        "q"
    }
}

impl<'a> ElementBuilder<'a> for Q {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
