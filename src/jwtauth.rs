use axum::http;
use jsonwebtoken as jwt;
// use async_trait::async_trait;
// use axum::extract::FromRequestParts;
// use axum_auth::{AuthBearerCustom, Rejection};
use axum_auth::AuthBearerCustom;
use http::request::Parts;

struct Jwt<C> {
    _data: jwt::TokenData<C>,
}

#[async_trait::async_trait]
impl<B, C> axum::extract::FromRequestParts<B> for Jwt<C>
where
    B: Send + Sync,
    C: Default,
{
    type Rejection = axum_auth::Rejection;

    async fn from_request_parts(parts: &mut Parts, _: &B) -> Result<Self, Self::Rejection> {
        Self::decode_request_parts(parts)
    }
}

impl<C> AuthBearerCustom for Jwt<C>
where
    C: Default,
{
    const ERROR_CODE: http::StatusCode = http::StatusCode::UNAUTHORIZED;

    const ERROR_OVERWRITE: Option<&'static str> = None;

    fn from_header(_contents: &str) -> Self {
        Jwt::<C>::default()
    }

    // fn decode_request_parts(req: &mut Parts) -> Result<Self, axum_auth::Rejection> {
    //     // Get authorization header
    //     let authorization = req
    //         .headers
    //         .get(http::header::AUTHORIZATION)
    //         .ok_or((Self::ERROR_CODE, ERR_MISSING))?
    //         .to_str()
    //         .map_err(|_| (Self::ERROR_CODE, ERR_CHARS))?;

    //     // Check that its a well-formed bearer and return
    //     let split = authorization.split_once(' ');
    //     match split {
    //         // Found proper bearer
    //         Some((name, contents)) if name == "Bearer" => Ok(Self::from_header(contents)),
    //         // Found empty bearer; sometimes request libraries format them as this
    //         _ if authorization == "Bearer" => Ok(Self::from_header("")),
    //         // Found nothing
    //         _ => Err((Self::ERROR_CODE, ERR_WRONG_BEARER)),
    //     }
    // }
}

impl<C> Default for Jwt<C>
where
    C: Default,
{
    fn default() -> Self {
        let data = jwt::TokenData {
            header: jwt::Header::default(),
            claims: C::default(),
        };
        Self { _data: data }
    }
}
