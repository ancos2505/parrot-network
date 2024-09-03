use std::{borrow::Cow, fmt::Display};

use crate::{elements::Style, OUTPUT_IDENTATION};

use super::Tag;

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlStyle<'a> {
    tag: Tag,
    depth: usize,
    contents: Cow<'a, str>,
}

impl Display for HtmlStyle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>\n", self.tag).as_str());

        output.push_str(&self.contents);

        output.push_str(format!("\n{iden}</{}>", self.tag.element.name()).as_str());
        write!(f, "{output}")
    }
}

impl<'a> HtmlStyle<'a> {
    pub fn new<S: AsRef<str>>(css: S) -> HtmlStyle<'a> {
        Self {
            tag: Tag {
                element: Box::new(Style),
                attrs: Default::default(),
            },
            depth: 1,
            contents: Cow::from(css.as_ref().to_owned()),
        }
    }
}

pub fn style<'a, S: AsRef<str>>(css: S) -> HtmlStyle<'a> {
    HtmlStyle::new(css)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_style() {
        let style = style("body { color: #000000; }");
        //dbg!(&style);
        println!("{style}");
    }
}
