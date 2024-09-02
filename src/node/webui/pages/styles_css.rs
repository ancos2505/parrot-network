use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use super::WebUiResponse;

pub(crate) fn styles_css() -> H10LibResult<WebUiResponse> {
    let css = include_str!("../../../../assets/styles.css");

    Ok(WebUiResponse::new(StatusCode::OK)
        .add_header(ContentType::css())
        .add_header(Date::now()?)
        .add_header(Server::default())
        .add_header(Pragma::default())
        .body(css))
}
