use super::*;

pub static X_FORWARDED_URI: headers::HeaderName =
    headers::HeaderName::from_static("x-forwarded-uri");

#[derive(Debug)]
pub struct XForwardedUri(http::Uri);

impl XForwardedUri {}

impl headers::Header for XForwardedUri {
    fn name() -> &'static headers::HeaderName {
        &X_FORWARDED_URI
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .parse()
            .map_err(|_| headers::Error::invalid())
            .map(Self)
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<http::HeaderValue>,
    {
        let text = self.0.to_string();
        let value = headers::HeaderValue::from_str(&text);
        values.extend(value);
    }
}
