use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Aside;
impl ElementName for Aside {
    fn name(&self) -> &'static str {
        "aside"
    }
}

impl<'a> ElementBuilder<'a> for Aside {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
