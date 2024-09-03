use crate::Tag;

use super::{ElementBuilder, ElementName, HtmlElement};

#[derive(Debug, PartialEq, Eq)]
pub struct Article;
impl ElementName for Article {
    fn name(&self) -> &'static str {
        "article"
    }
}

impl<'a> ElementBuilder<'a> for Article {
    fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            element: Box::new(Self),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
