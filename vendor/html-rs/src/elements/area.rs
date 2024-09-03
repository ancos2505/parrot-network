use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Area;
impl ElementName for Area {
    fn name(&self) -> &'static str {
        "area"
    }
}

impl<'a> ElementBuilder<'a> for Area {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
