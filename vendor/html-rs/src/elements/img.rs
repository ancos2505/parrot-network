use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Img;
impl ElementName for Img {
    fn name(&self) -> &'static str {
        "img"
    }
}

impl<'a> ElementBuilder<'a> for Img {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
