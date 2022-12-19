use super::*;

pub static X_FORWARDED_METHOD: headers::HeaderName =
    headers::HeaderName::from_static("x-forwarded-method");
// const X_FORWARDED_METHOD_REF: &headers::HeaderName = &X_FORWARDED_METHOD;

// pub type XForwardedMethod = TypedHeader<ForwardedMethod>;

#[derive(Debug)]
pub struct XForwardedMethod(http::Method);

impl XForwardedMethod {
    // const NAME: headers::HeaderName = headers::HeaderName::from_static("x-forwarded-method");
}

impl headers::Header for XForwardedMethod {
    fn name() -> &'static headers::HeaderName {
        &X_FORWARDED_METHOD
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        let method =
            http::Method::from_bytes(value.as_bytes()).map_err(|_| headers::Error::invalid())?;
        Ok(Self(method))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        let text = match self.0 {
            http::Method::CONNECT => "CONNECT",
            http::Method::DELETE => "DELETE",
            http::Method::GET => "GET",
            http::Method::HEAD => "HEAD",
            http::Method::OPTIONS => "OPTIONS",
            http::Method::PATCH => "PATCH",
            http::Method::POST => "POST",
            http::Method::PUT => "PUT",
            http::Method::TRACE => "TRACE",
            _ => unreachable!(),
        };

        let value = headers::HeaderValue::from_static(text);
        values.extend(iter::once(value));
    }
}
