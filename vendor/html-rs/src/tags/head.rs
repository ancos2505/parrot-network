use std::fmt::Display;

use crate::{
    elements::{Head, HtmlElement},
    OUTPUT_IDENTATION,
};

use super::Tag;

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlHead<'a> {
    pub tag: Tag,
    pub depth: usize,
    pub items: Vec<HtmlElement<'a>>,
}
impl Default for HtmlHead<'_> {
    fn default() -> Self {
        Self {
            tag: Tag {
                element: Box::new(Head),
                attrs: Default::default(),
            },
            depth: 1,
            items: Default::default(),
        }
    }
}
impl Display for HtmlHead<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>", self.tag).as_str());
        for item in &self.items {
            output.push_str(format!("{}", item).as_str());
        }
        output.push_str(format!("\n{iden}</{}>", self.tag.element.name()).as_str());
        write!(f, "{output}")
    }
}

impl<'a> HtmlHead<'a> {
    pub const fn as_str() -> &'static str {
        "head"
    }
    pub fn builder() -> HtmlHead<'a> {
        Default::default()
    }
    pub fn add(mut self, tag: HtmlElement<'a>) -> HtmlHead<'a> {
        self.items.push(tag.into());
        Self {
            tag: self.tag,
            depth: self.depth,
            items: self.items,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{elements::Title, Html};

    use super::*;

    #[test]
    fn ok_on_build_simple_head() {
        let title = Title::text("Some title");
        let head = Html::builder().head_item(title);

        //dbg!(&head);
        println!("{head}");
    }
}
