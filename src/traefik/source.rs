use super::*;

pub static X_FORWARDED_FOR: headers::HeaderName =
    headers::HeaderName::from_static("x-forwarded-for");

#[derive(Debug)]
pub struct XForwardedFor(net::IpAddr);

impl headers::Header for XForwardedFor {
    fn name() -> &'static headers::HeaderName {
        &X_FORWARDED_FOR
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

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        let text = self.0.to_string();
        let value = headers::HeaderValue::from_str(&text);
        values.extend(value);
    }
}
