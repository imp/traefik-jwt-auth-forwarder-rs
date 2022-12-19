use super::*;

pub static X_FORWARDED_PROTO: headers::HeaderName =
    headers::HeaderName::from_static("x-forwarded-proto");

#[derive(Debug)]
pub struct XForwardedProto(http::uri::Scheme);

impl XForwardedProto {}

impl headers::Header for XForwardedProto {
    fn name() -> &'static headers::HeaderName {
        &X_FORWARDED_PROTO
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;
        let proto = value
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .parse()
            .map_err(|_| headers::Error::invalid())?;
        Ok(Self(proto))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<http::HeaderValue>,
    {
        // let text = match self.0 {
        //     http::uri::Scheme::HTTP => "http",
        //     http::uri::Scheme::HTTPS => "https",
        //     // _ => unreachable!(),
        // };

        let text = if self.0 == http::uri::Scheme::HTTP {
            "http"
        } else if self.0 == http::uri::Scheme::HTTPS {
            "https"
        } else {
            unreachable!()
        };
        let value = headers::HeaderValue::from_static(text);
        values.extend(iter::once(value));
    }
}
