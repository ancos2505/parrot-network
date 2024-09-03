//! # 7.  Entity
//!
//!  Full-Request and Full-Response messages may transfer an entity within some
//! requests and responses. An entity consists of Entity-Header fields and
//! (usually) an Entity-Body. In this section, both sender and recipient refer
//! to either the client or the server, depending on who sends and who receives
//! the entity.
//!
//! ## 7.1  Entity Header Fields
//!
//!  Entity-Header fields define optional metainformation about the Entity-Body
//! or, if no body is present, about the resource identified by the request.
//!
//! **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-7
//!

mod allow;
mod content_encoding;
mod content_length;
mod content_type;
mod expires;
mod last_modified;

pub use self::{
    allow::Allow, content_encoding::ContentEncoding, content_length::ContentLength,
    content_type::ContentType, expires::Expires, last_modified::LastModified,
};
