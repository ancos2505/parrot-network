use std::sync::atomic::Ordering;

use html_rs::{
    elements::{Div, ElementBuilder, Form, Input, Link, Meta, TextContent, Title, H1, P},
    Html, HtmlBody,
};

use h10::http::{
    headers::{ContentType, Date, Location, Pragma, Server},
    request::Request,
    result::H10LibResult,
    status_code::StatusCode,
};

use super::WebUiResponse;

use crate::ROOT_PAGER_COUNTER;

pub(crate) fn root(request: Request) -> H10LibResult<WebUiResponse> {
    if let Some(endpoint) = request.query_string.get("endpoint") {
        match &**endpoint.value() {
            "counter" => {
                let _ = ROOT_PAGER_COUNTER.fetch_add(1, Ordering::SeqCst);
                return Ok(WebUiResponse::new(StatusCode::MovedTemporarily)
                    .add_header(Location::from_str("/")?));
            }
            _ => (),
        }
    }

    let current_counter = ROOT_PAGER_COUNTER.load(Ordering::SeqCst);

    let favicon_disabled = Link::builder()
        .attr("rel", "shortcut icon")
        .attr("href", "data:image/x-icon;,")
        .attr("type", "image/x-icon");
    let form = Form::builder()
        .attr("action", "")
        .attr("method", "get")
        .append_child(
            Input::builder()
                .attr("type", "hidden")
                .attr("name", "endpoint")
                .attr("value", "counter"),
        )
        .append_child(
            Input::builder()
                .attr("id", "btn-counter")
                .attr("type", "submit")
                .attr("value", format!("count is {current_counter}")),
        );
    let card = Div::builder().attr("class", "card").append_child(form);

    let div = Div::builder().append_child(
        Div::builder()
            .append_child(H1::builder().append_child(TextContent::text("It works!")))
            .append_child(card)
            .append_child(P::builder().append_child(TextContent::text("You can disable <strong>Javascript</strong> in your browser and the app still works!"))),
    );
    let html = Html::builder()
        .head_item(favicon_disabled)
        .head_item(Title::builder().append_child(TextContent::text(format!(
            "{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))))
        .head_item(Meta::builder().attr("charset", "utf-8"))
        .head_item(
            Meta::builder()
                .attr("name", "viewport")
                .attr("content", "width=device-width, initial-scale=1.0"),
        )
        .head_item(
            Link::builder()
                .attr("href", "/assets/styles.css")
                .attr("rel", "stylesheet")
                .attr("type", "text/css"),
        )
        .body(
            HtmlBody::builder()
                .set_attr("lang", "en")
                .set_attr("server-name", env!("CARGO_PKG_NAME"))
                .set_attr("server-version", env!("CARGO_PKG_VERSION"))
                .append_child(Div::builder().attr("id", "app").append_child(div)),
        );

    #[cfg(feature = "debug")]
    println!("{html:?}");

    Ok(WebUiResponse::new(StatusCode::OK)
        .add_header(ContentType::html())
        .add_header(Date::now()?)
        .add_header(Server::default())
        .add_header(Pragma::default())
        .body(html.to_string()))
}
