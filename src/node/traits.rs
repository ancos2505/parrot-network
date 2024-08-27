use h10::http::response::Response;

pub(crate) trait IntoResponse {
    fn into_response(self) -> Response;
}
