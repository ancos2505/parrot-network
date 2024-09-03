use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Footer;
impl ElementName for Footer {
    fn name(&self) -> &'static str {
        "footer"
    }
}

impl<'a> ElementBuilder<'a> for Footer {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
