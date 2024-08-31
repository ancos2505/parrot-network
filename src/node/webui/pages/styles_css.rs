use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use super::WebuiResponse;

pub(crate) fn styles_css() -> H10LibResult<WebuiResponse> {
    let css = include_str!("../../../../assets/styles.css");

    Ok(WebuiResponse::new(StatusCode::OK)
        .add_header(ContentType::css())
        .add_header(Date::now()?)
        .add_header(Server::default())
        .add_header(Pragma::default())
        .body(css))
}
