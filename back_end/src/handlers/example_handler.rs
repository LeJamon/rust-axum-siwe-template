use crate::models::{error::AppError, jwt::validate_jwt};
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};

pub async fn handle_example_request(
    headers: HeaderMap,
) -> Result<(StatusCode, Json<String>), AppError> {
    let authorization = headers
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let claims = validate_jwt(authorization)
        .await
        .map_err(|_| AppError::Unauthorized)?;
    let owner = format!("0x{}", claims.address);
    Ok((StatusCode::OK, Json(owner)))
}
