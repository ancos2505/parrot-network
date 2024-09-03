use html_rs::{
    elements::{Div, ElementBuilder, TextContent, Title, P},
    Html, HtmlBody, HtmlScript, HtmlStyle,
};

fn main() {
    let title = Title::builder().append_child(TextContent::text("It works!"));
    let style = HtmlStyle::new("body { color: #000000; }");
    let script1 = HtmlScript::new(
        format!(
            r#"console.log("Hello from file {} at line {}")"#,
            file!(),
            line!(),
        )
        .as_str(),
    );

    let div = Div::builder().attr("class", "light-theme").append_child(
        P::builder()
            .attr("class", "light-theme")
            .append_child(TextContent::text("It Works!")),
    );

    let body = HtmlBody::builder()
        .set_attr("lang", "en")
        .script(script1)
        .append_child(div);

    let script2 = HtmlScript::new(
        format!(
            r#"console.log("Hello from file {} at line {}")"#,
            file!(),
            line!(),
        )
        .as_str(),
    );
    let html = Html::builder()
        .head_item(title)
        .add_style(style)
        .add_script(script2)
        .body(body);

    #[cfg(feature = "debug")]
    dbg!(&html);

    println!("{html}");
}
