use std::fmt::Display;

use crate::{
    elements::{Body, HtmlElement},
    OUTPUT_IDENTATION,
};

use super::{script::HtmlScript, Tag};

#[derive(Debug)]
pub struct HtmlBody<'a> {
    tag: Tag,
    depth: usize,
    elements: Vec<HtmlElement<'a>>,
    script: Vec<HtmlScript<'a>>,
}

impl Default for HtmlBody<'_> {
    fn default() -> Self {
        Self {
            tag: Tag {
                element: Box::new(Body),
                attrs: Default::default(),
            },
            depth: 1,
            elements: Default::default(),
            script: Default::default(),
        }
    }
}

impl Display for HtmlBody<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>", self.tag).as_str());
        for elem in self.elements.iter() {
            output.push_str(format!("{iden}{}", elem).as_str());
        }
        output.push_str(format!("\n{iden}</{}>", self.tag.element.name()).as_str());
        write!(f, "{output}")
    }
}

impl<'a> HtmlBody<'a> {
    pub fn builder() -> HtmlBody<'a> {
        Default::default()
    }
    pub fn script(mut self, script: HtmlScript<'a>) -> HtmlBody<'a> {
        self.script.push(script);
        HtmlBody {
            tag: self.tag,
            depth: self.depth,
            script: self.script,
            elements: self.elements,
        }
    }
    pub fn append_child(mut self, mut element: HtmlElement<'a>) -> HtmlBody<'a> {
        element.set_depth(self.depth + 1);
        self.elements.push(element);
        HtmlBody {
            tag: self.tag,
            depth: self.depth,
            script: self.script,
            elements: self.elements,
        }
    }
    pub fn set_attr<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> HtmlBody<'a> {
        self.tag.set_attr(key, value);
        HtmlBody {
            tag: self.tag,
            depth: self.depth,
            script: self.script,
            elements: self.elements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_body() {
        let body = HtmlBody::builder().script(
            HtmlScript::new(format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            ))
            .into(),
        );
        //dbg!(&body);
        println!("{body}");
    }
}
