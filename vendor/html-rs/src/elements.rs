mod a;
mod abbr;
mod address;
mod area;
mod article;
mod aside;
mod audio;
mod b;
mod base;
mod bdi;
mod bdo;
mod blockquote;
mod body;
mod br;
mod button;
mod canvas;
mod caption;
mod cite;
mod code;
mod col;
mod colgroup;
mod data;
mod datalist;
mod dd;
mod del;
mod details;
mod dfn;
mod dialog;
mod div;
mod dl;
mod dt;
mod em;
mod embed;
mod fieldset;
mod figcaption;
mod figure;
mod footer;
mod form;
mod h1;
mod h2;
mod h3;
mod h4;
mod h5;
mod h6;
mod head;
mod header;
mod hgroup;
mod hr;
mod html;
mod i;
mod iframe;
mod img;
mod innerhtml;
mod input;
mod ins;
mod kbd;
mod label;
mod legend;
mod li;
mod link;
mod main;
mod map;
mod mark;
mod marquee;
mod menu;
mod meta;
mod meter;
mod nav;
mod noscript;
mod object;
mod ol;
mod optgroup;
mod option;
mod outerhtml;
mod output;
mod p;
mod picture;
mod pre;
mod progress;
mod q;
mod rp;
mod rt;
mod ruby;
mod s;
mod samp;
mod script;
mod search;
mod section;
mod select;
mod slot;
mod small;
mod source;
mod span;
mod strong;
mod style;
mod sub;
mod summary;
mod table;
mod tbody;
mod td;
mod template;
mod text;
mod textarea;
mod tfoot;
mod th;
mod thead;
mod time;
mod title;
mod tr;
mod track;
mod u;
mod ul;
mod var;
mod video;
mod wbr;

use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

pub use self::{
    a::A, abbr::Abbr, address::Address, area::Area, article::Article, aside::Aside, audio::Audio,
    b::B, base::Base, bdi::Bdi, bdo::Bdo, blockquote::Blockquote, body::Body, br::Br,
    button::Button, canvas::Canvas, caption::Caption, cite::Cite, code::Code, col::Col,
    colgroup::Colgroup, data::Data, datalist::Datalist, dd::Dd, del::Del, details::Details,
    dfn::Dfn, dialog::Dialog, div::Div, dl::Dl, dt::Dt, em::Em, embed::Embed, fieldset::Fieldset,
    figcaption::Figcaption, figure::Figure, footer::Footer, form::Form, h1::H1, h2::H2, h3::H3,
    h4::H4, h5::H5, h6::H6, head::Head, header::Header, hgroup::Hgroup, hr::Hr, html::Html, i::I,
    iframe::Iframe, img::Img, innerhtml::Innerhtml, input::Input, ins::Ins, kbd::Kbd, label::Label,
    legend::Legend, li::Li, link::Link, main::Main, map::Map, mark::Mark, marquee::Marquee,
    menu::Menu, meta::Meta, meter::Meter, nav::Nav, noscript::Noscript, object::Object, ol::Ol,
    optgroup::Optgroup, option::OptionElement, outerhtml::Outerhtml, output::Output, p::P,
    picture::Picture, pre::Pre, progress::Progress, q::Q, rp::Rp, rt::Rt, ruby::Ruby, s::S,
    samp::Samp, script::Script, search::Search, section::Section, select::Select, slot::Slot,
    small::Small, source::Source, span::Span, strong::Strong, style::Style, sub::Sub,
    summary::Summary, table::Table, tbody::Tbody, td::Td, template::Template, text::TextContent,
    textarea::Textarea, tfoot::Tfoot, th::Th, thead::Thead, time::Time, title::Title, tr::Tr,
    track::Track, u::U, ul::Ul, var::Var, video::Video, wbr::Wbr,
};

use crate::{tags::Tag, OUTPUT_IDENTATION};

pub trait ElementName: Debug {
    fn name(&self) -> &'static str;
}

pub trait ElementBuilder<'a> {
    fn builder() -> HtmlElement<'a>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum HtmlElementChildren<'a> {
    TextContent(Cow<'a, str>),
    Children(Vec<HtmlElement<'a>>),
}

impl Display for HtmlElementChildren<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            HtmlElementChildren::TextContent(text) => text.to_owned(),
            HtmlElementChildren::Children(html_elements) => {
                let mut output = "".to_owned();
                for elem in html_elements {
                    output.push_str(elem.to_string().as_str())
                }
                output.into()
            }
        };

        write!(f, "{output}")
    }
}

#[derive(Debug)]
pub struct HtmlElement<'a> {
    pub tag: Tag,
    depth: usize,
    pub children: Option<HtmlElementChildren<'a>>,
}

impl Display for HtmlElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);

        let tagname_and_attrs = format!("{}", self.tag);
        let tag_name = &self.tag.element.name();

        match &self.children {
            Some(children) => {
                if let HtmlElementChildren::TextContent(s) = children {
                    let text_iden = " ".repeat(OUTPUT_IDENTATION * (self.depth + 1));
                    output.push_str(format!("\n{text_iden}{s}").as_str())
                } else {
                    output.push_str(
                        format!("\n{iden}<{tagname_and_attrs}>{children}\n{iden}</{tag_name}>")
                            .as_str(),
                    )
                }
            }
            None => output.push_str(format!("\n{iden}<{tagname_and_attrs}></{tag_name}>").as_str()),
        };

        write!(f, "{output}")
    }
}
impl PartialEq for HtmlElement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.depth == other.depth && self.children == other.children
    }
}

impl Eq for HtmlElement<'_> {}
impl<'a> HtmlElement<'a> {
    pub fn builder(tag: Tag) -> HtmlElement<'a> {
        HtmlElement {
            tag,
            // It exists from HtmlBody(depth=2)
            depth: 2,
            children: Default::default(),
        }
    }
    /// ## Attention: Only use in TextContent
    pub fn inner_text<S: AsRef<str>>(mut self, text: S) -> HtmlElement<'a> {
        self.children = Some(HtmlElementChildren::TextContent(
            text.as_ref().to_owned().into(),
        ));

        HtmlElement {
            tag: self.tag,
            depth: self.depth,
            children: self.children,
        }
    }
    pub fn attr<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> HtmlElement<'a> {
        self.tag.set_attr(key, value);

        HtmlElement {
            tag: self.tag,
            depth: self.depth,
            children: self.children,
        }
    }
    pub fn append_child(self, mut new_element: HtmlElement<'a>) -> HtmlElement<'a> {
        new_element.depth = self.depth + 1;
        // dbg!(&new_element);
        if let Some(children) = self.children {
            let new_children = match children {
                HtmlElementChildren::TextContent(text) => {
                    let migrated = TextContent::text(text);
                    HtmlElementChildren::Children(vec![migrated, new_element])
                }
                HtmlElementChildren::Children(mut html_elements) => {
                    html_elements.push(new_element);
                    HtmlElementChildren::Children(html_elements)
                }
            };
            HtmlElement {
                tag: self.tag,
                depth: self.depth,
                children: Some(new_children),
            }
        } else {
            HtmlElement {
                tag: self.tag,
                depth: self.depth,
                children: Some(HtmlElementChildren::Children(vec![new_element])),
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }
}

#[cfg(test)]
mod tests {
    use text::TextContent;

    use super::*;
    use crate::elements::{div::Div, p::P};
    #[test]
    fn ok_on_build_div_with_paragraph() {
        let div = Div::builder().attr("class", "light-theme").append_child(
            P::builder()
                .attr("class", "light-theme")
                .append_child(TextContent::text("It Works!")),
        );

        println!("{div}");
    }
}
