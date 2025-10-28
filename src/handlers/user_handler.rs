use axum::{extract::State, response::IntoResponse, Json};
use tracing::info;

use crate::{error::AppError, models::user_model::User, AppState};
#[allow(dead_code)]
pub(crate) async fn create_test_user(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    let msg = state.create_user(user).await?;
    info!("返回数据:{:?}", Json(&msg));
    Ok(Json(msg))
}
#[allow(dead_code)]
pub(crate) async fn check_user_one(
    State(state): State<AppState>,
    Json(id): Json<i32>,
) -> Result<impl IntoResponse, AppError> {
    let msg = state.check_user(id).await?;
    info!("返回数据:{:?}", Json(&msg));
    Ok(Json(msg))
}
