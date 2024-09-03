use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Select;
impl ElementName for Select {
    fn name(&self) -> &'static str {
        "select"
    }
}

impl<'a> ElementBuilder<'a> for Select {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
