use std::env;
use std::net::Ipv4Addr;

use axum::http;
use axum::response::IntoResponse;
use axum::routing::{any, get, Router};
use axum::TypedHeader;
use axum_auth::AuthBearer;
use jsonwebtoken as jwt;
use once_cell::sync::Lazy;

use claims::Claims;

mod claims;
mod jwtauth;
mod traefik;

static PUBLIC_KEY: Lazy<jwt::DecodingKey> = Lazy::new(load_key);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/auth", any(auth))
        .route("/healthz", get(|| async { "healthy" }))
        .route("/ready", get(|| async { "ready" }));

    let addr = (Ipv4Addr::LOCALHOST, 8000).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Hello, world!");
}

#[axum::debug_handler]
async fn auth(
    TypedHeader(method): TypedHeader<traefik::XForwardedMethod>,
    TypedHeader(proto): TypedHeader<traefik::XForwardedProto>,
    AuthBearer(token): AuthBearer,
) -> impl IntoResponse {
    tracing::debug!(token, "Auth Bearer");
    tracing::debug!(?method, ?proto);

    let v = jwt::Validation::new(jwt::Algorithm::ES384);
    let data = jwt::decode::<Claims>(&token, &PUBLIC_KEY, &v).expect("Failed to validate");
    let claims = data.claims;

    if let Ok(claims) = claims.validate() {
        Ok((claims.into_headers(), "OK"))
    } else {
        Err(http::StatusCode::FORBIDDEN)
    }
}

fn load_key() -> jwt::DecodingKey {
    let text = env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY env is missing");
    jwt::DecodingKey::from_ec_pem(text.as_bytes()).expect("Failed to load ECDSA Key")
}
