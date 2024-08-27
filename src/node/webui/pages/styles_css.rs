use h10::http::{
    headers::{ContentType, Date, Pragma, Server},
    result::H10LibResult,
    status_code::StatusCode,
};

use crate::node::webui::ServerResponse;

pub(crate) fn styles_css() -> H10LibResult<ServerResponse> {
    let css = include_str!("../../../../assets/styles.css");

    Ok(ServerResponse::new(StatusCode::OK)
        .header(ContentType::css())
        .header(Date::now()?)
        .header(Server::default())
        .header(Pragma::default())
        .body(css))
}
