//! Request Header Fields
//!
//! Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-5.2

mod authorization;
mod from;
mod if_modified_since;
mod referer;
mod user_agent;

pub use self::{
    authorization::Authorization, from::From, if_modified_since::IfModifiedSince, referer::Referer,
    user_agent::UserAgent,
};
