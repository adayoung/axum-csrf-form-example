use askama_axum::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Form, Router};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    csrf_token: String,
}

#[derive(Deserialize, Serialize)]
struct DataForm {
    csrf_token: String,
}

#[tokio::main]
async fn main() {
    let csrf_key = axum_csrf::Key::from(
        "bREppo26eIkGETFbpvHOP8SFBF0WqeNbiNxrccb417ItZhGFssK1roLtN8SbKvRj".as_bytes(),
    );
    let csrf_config = CsrfConfig::new()
        .with_cookie_name("csrf")
        .with_cookie_path("/")
        .with_key(Some(csrf_key));

    let app = Router::new()
        .route("/", get(set_token).post(validate_token))
        .layer(CsrfLayer::new(csrf_config));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:2024")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn set_token(token: CsrfToken) -> impl IntoResponse {
    let template = IndexTemplate {
        csrf_token: token.authenticity_token().unwrap(),
    };

    (token, template)
}

async fn validate_token(token: CsrfToken, Form(payload): Form<DataForm>) -> impl IntoResponse {
    if token.verify(&payload.csrf_token).is_err() {
        return (StatusCode::FORBIDDEN, "CSRF token is NOT valid!").into_response();
    }

    (StatusCode::OK, "CSRF token is valid!").into_response()
}
