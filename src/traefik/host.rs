use super::*;

pub static X_FORWARDED_HOST: headers::HeaderName =
    headers::HeaderName::from_static("x-forwarded-host");

#[derive(Debug)]
pub struct XForwardedHost(String);

impl XForwardedHost {
    // const NAME: headers::HeaderName = headers::HeaderName::from_static("x-forwarded-method");
}

impl headers::Header for XForwardedHost {
    fn name() -> &'static headers::HeaderName {
        &X_FORWARDED_HOST
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        let host = value
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();
        Ok(Self(host))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        let value = headers::HeaderValue::from_str(&self.0);
        values.extend(value);
    }
}
