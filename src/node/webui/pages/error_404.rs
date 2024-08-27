use html_rs::{
    elements::{ElementBuilder, Link, Meta, TextContent, Title, H1},
    Html, HtmlBody,
};

use h10::http::{
    headers::{ContentType, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use crate::node::webui::ServerResponse;

pub(crate) fn error_404() -> H10LibResult<ServerResponse> {
    let favicon_disabled = Link::builder()
        .attr("rel", "shortcut icon")
        .attr("href", "data:image/x-icon;,")
        .attr("type", "image/x-icon");
    let html = Html::builder()
        .head_item(Title::builder().append_child(TextContent::text("Not Found")))
        .head_item(favicon_disabled)
        .head_item(Meta::builder().attr("charset", "utf-8"))
        .body(
            HtmlBody::builder()
                .set_attr("lang", "en")
                .set_attr("server-name", env!("CARGO_PKG_NAME"))
                .set_attr("server-version", env!("CARGO_PKG_VERSION"))
                .append_child(H1::builder().append_child(TextContent::text("Not Found"))),
        );

    #[cfg(feature = "debug")]
    println!("{html:?}");

    Ok(ServerResponse::new(StatusCode::NotFound)
        .header(ContentType::html())
        .header(Server::default())
        .header(Pragma::default())
        .body(html))
}
