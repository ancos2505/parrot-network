use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Menu;
impl ElementName for Menu {
    fn name(&self) -> &'static str {
        "menu"
    }
}

impl<'a> ElementBuilder<'a> for Menu {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
