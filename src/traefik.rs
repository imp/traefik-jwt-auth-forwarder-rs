use std::iter;

use axum::headers;
use axum::http;

pub use method::XForwardedMethod;
pub use proto::XForwardedProto;

mod method;
mod proto;
