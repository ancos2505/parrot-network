use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Cite;
impl ElementName for Cite {
    fn name(&self) -> &'static str {
        "cite"
    }
}

impl<'a> ElementBuilder<'a> for Cite {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
