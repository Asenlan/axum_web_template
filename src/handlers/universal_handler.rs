use axum::extract::State;
use axum::response::IntoResponse;
use crate::error::AppError;
use crate::AppState;

pub(crate) async fn shutdown_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    state.cancel_token.cancel();
    Ok("定时任务关闭信号".to_string())
}
