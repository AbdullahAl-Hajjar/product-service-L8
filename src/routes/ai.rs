use crate::startup::AppState;
use actix_web::{error, http::StatusCode, web, Error, HttpResponse};
use reqwest::Client;
use std::time::Duration;

async fn forward(request: reqwest::RequestBuilder) -> Result<HttpResponse, Error> {
    let response = request
        .timeout(Duration::from_secs(165))
        .send()
        .await
        .map_err(|_| error::ErrorBadGateway("Could not reach the AI service"))?;
    let status =
        StatusCode::from_u16(response.status().as_u16()).map_err(error::ErrorBadGateway)?;
    let body = response
        .bytes()
        .await
        .map_err(|_| error::ErrorBadGateway("Could not read the AI service response"))?;
    Ok(HttpResponse::build(status)
        .content_type("application/json")
        .body(body))
}

pub async fn ai_health(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    forward(Client::new().get(format!("{}/health", data.settings.ai_service_url))).await
}

pub async fn ai_generate_description(
    data: web::Data<AppState>,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let url = format!("{}/generate/description", data.settings.ai_service_url);
    forward(
        Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body(body),
    )
    .await
}

pub async fn ai_generate_image(
    data: web::Data<AppState>,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let url = format!("{}/generate/image", data.settings.ai_service_url);
    forward(
        Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body(body),
    )
    .await
}
