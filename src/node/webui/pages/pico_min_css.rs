use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use super::WebUiResponse;

pub(crate) fn pico_min_css() -> H10LibResult<WebUiResponse> {
    let css = include_str!("../../../../assets/pico.min.css");

    Ok(WebUiResponse::new(StatusCode::OK)
        .add_header(ContentType::css())
        .add_header(Date::now()?)
        .add_header(Server::default())
        .add_header(Pragma::default())
        .set_body(css))
}
