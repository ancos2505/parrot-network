use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct OptionElement;
impl ElementName for OptionElement {
    fn name(&self) -> &'static str {
        "option"
    }
}

impl<'a> ElementBuilder<'a> for OptionElement {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
