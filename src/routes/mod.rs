use crate::handlers::universal_handler::*;
use crate::handlers::user_handler::*;
use crate::middlewares::set_layer;
use crate::AppState;
use anyhow::{Error, Result};
use axum::routing::get;
use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
pub async fn get_router(state: AppState) -> Result<Router, Error> {
    // 数据格式处理接口
    // let exd = Router::new().route("/wash_header", post(wash_header_handler));
    //数据模型接口
    // let exm = Router::new().route(
    //     "/gambler_have_card_model_handler",
    //     post(gambler_have_card_model_handler),
    // );

    //数据下钻
    // let drill = Router::new().route(
    //     "/gambler_no_card_drill",
    //     post(gambler_no_card_drill_handler),
    // );

    let api = Router::new()
        // .nest("/exd", exd)
        // .nest("/exm", exm)
        // .nest("/drill", drill)
        .route("/fetch_page", post(create_test_user))
        .route("/shutdown_task", get(shutdown_handler));
    // .route("/fetch_graph_all", post(fetch_graph_all_handler))
    // .route("/fetch_graph_agg", post(fetch_graph_agg_handler))
    // .route("/shutdown_handler", get(shutdown_handler));

    let app = Router::new()
        .nest("/ay", api)
        .fallback(handler_404)
        .with_state(state);
    Ok(set_layer(app))
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
