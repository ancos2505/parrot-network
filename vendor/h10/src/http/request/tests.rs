use crate::http::{body::Body, url_path::UrlPath};

use super::Request;

#[test]
fn build_simple_request_and_serialize_using_method_delete() {
    let expected_str = format!(
        "DELETE / HTTP/1.0\r\nUser-Agent: {}/{}\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let request = Request::delete().path(UrlPath::root()).finish();

    assert_eq!(expected_str, &*request.to_string())
}

#[test]
fn build_simple_request_and_serialize_using_method_get() {
    let expected_str = format!(
        "GET / HTTP/1.0\r\nUser-Agent: {}/{}\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let request = Request::get().path(UrlPath::root()).finish();

    assert_eq!(expected_str, &*request.to_string())
}

#[test]
fn build_simple_request_and_serialize_using_method_head() {
    let expected_str = format!(
        "HEAD / HTTP/1.0\r\nUser-Agent: {}/{}\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let request = Request::head().path(UrlPath::root()).finish();

    assert_eq!(expected_str, &*request.to_string())
}
#[test]
fn build_simple_request_and_serialize_using_method_link() {
    let expected_str = format!(
        "LINK / HTTP/1.0\r\nUser-Agent: {}/{}\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let request = Request::link().path(UrlPath::root()).finish();

    assert_eq!(expected_str, &*request.to_string())
}

#[test]
fn build_simple_request_and_serialize_using_method_post() {
    let expected_str = format!(
        "POST / HTTP/1.0\r\nUser-Agent: {}/{}\r\nContent-Length: 0\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let request = Request::post()
        .path(UrlPath::root())
        .set_body(Body::empty())
        .finish();

    assert_eq!(expected_str, &*request.to_string())
}

#[test]
fn build_simple_request_and_serialize_using_method_put() {
    let expected_str = format!(
        "PUT / HTTP/1.0\r\nUser-Agent: {}/{}\r\nContent-Length: 0\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let request = Request::put()
        .path(UrlPath::root())
        .set_body(Body::empty())
        .finish();

    assert_eq!(expected_str, &*request.to_string())
}

#[test]
fn build_simple_request_and_serialize_using_method_unlink() {
    let expected_str = format!(
        "UNLINK / HTTP/1.0\r\nUser-Agent: {}/{}\r\n\r\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let request = Request::unlink().path(UrlPath::root()).finish();

    assert_eq!(expected_str, &*request.to_string())
}
