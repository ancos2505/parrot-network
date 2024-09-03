mod body;
mod head;
mod html;
mod script;
mod style;

use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

use crate::elements::ElementName;

pub use self::{body::HtmlBody, head::HtmlHead, html::Html, script::HtmlScript, style::HtmlStyle};

#[derive(Debug)]
pub struct Tag {
    pub element: Box<dyn ElementName>,
    pub attrs: BTreeMap<String, String>,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.element.name() == other.element.name() && self.attrs == other.attrs
    }
}

impl Eq for Tag {}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("{}", self.element.name());
        let max_idx = self.attrs.len();
        if max_idx > 0 {
            output.push(' ');
        }
        for (idx, (name, value)) in self.attrs.iter().enumerate() {
            output.push_str(format!(r#"{name}="{value}""#).as_str());
            if idx + 1 < max_idx {
                output.push(' ')
            }
        }
        write!(f, "{output}")
    }
}
impl Tag {
    pub fn set_attr<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        let _b = self
            .attrs
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    #[ignore = "Todo"]
    fn ok_on_build_html_tag() {
        todo!()
    }
}
