use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Fieldset;
impl ElementName for Fieldset {
    fn name(&self) -> &'static str {
        "fieldset"
    }
}

impl<'a> ElementBuilder<'a> for Fieldset {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
