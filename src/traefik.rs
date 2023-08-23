use std::iter;
use std::net;

use axum::headers;
use axum::http;

pub use host::XForwardedHost;
pub use method::XForwardedMethod;
pub use proto::XForwardedProto;
pub use source::XForwardedFor;
pub use uri::XForwardedUri;

mod host;
mod method;
mod proto;
mod source;
mod uri;
