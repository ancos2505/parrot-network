use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Outerhtml;
impl ElementName for Outerhtml {
    fn name(&self) -> &'static str {
        "outerhtml"
    }
}

impl<'a> ElementBuilder<'a> for Outerhtml {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
