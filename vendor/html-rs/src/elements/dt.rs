use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Dt;
impl ElementName for Dt {
    fn name(&self) -> &'static str {
        "dt"
    }
}

impl<'a> ElementBuilder<'a> for Dt {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
