use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct H5;
impl ElementName for H5 {
    fn name(&self) -> &'static str {
        "h5"
    }
}

impl<'a> ElementBuilder<'a> for H5 {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
