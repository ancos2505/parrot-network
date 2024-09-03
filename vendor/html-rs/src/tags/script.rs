use std::{borrow::Cow, fmt::Display};

use crate::{elements::Script, OUTPUT_IDENTATION};

use super::Tag;

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlScript<'a> {
    tag: Tag,
    depth: usize,
    contents: Cow<'a, str>,
}

impl Display for HtmlScript<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>\n", self.tag).as_str());

        output.push_str(&self.contents);

        output.push_str(format!("\n{iden}</{}>", self.tag.element.name()).as_str());
        write!(f, "{output}")
    }
}

impl<'a> HtmlScript<'a> {
    pub fn new<S: AsRef<str>>(script: S) -> HtmlScript<'a> {
        Self {
            tag: Tag {
                element: Box::new(Script),
                attrs: Default::default(),
            },
            depth: 1,
            contents: Cow::from(script.as_ref().to_owned()),
        }
    }
}

pub fn script<'a, S: AsRef<str>>(script: S) -> HtmlScript<'a> {
    HtmlScript::new(script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_script() {
        let script = script("body { color: #000000; }");
        //dbg!(&script);
        println!("{script}");
    }
}
