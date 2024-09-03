use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Nav;
impl ElementName for Nav {
    fn name(&self) -> &'static str {
        "nav"
    }
}

impl<'a> ElementBuilder<'a> for Nav {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
