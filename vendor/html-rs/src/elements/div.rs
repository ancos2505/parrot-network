use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Div;
impl ElementName for Div {
    fn name(&self) -> &'static str {
        "div"
    }
}

impl<'a> ElementBuilder<'a> for Div {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
