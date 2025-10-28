use show_template::{get_router, start_timer, AppConfig, AppState};
use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};
#[tokio::main]
async fn main() -> Result<()> {
    // console layer for tracing-subscriber
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    // file appender layer for tracing-subscriber
    let file_appender = tracing_appender::rolling::daily(r#"logs"#, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file = fmt::Layer::new()
        .with_ansi(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(file)
        .with(console)
        .init();

    let config = AppConfig::load()?;
    let addr = format!("0.0.0.0:{}", config.server.port);
    let state = AppState::try_new(config).await?;
    let app = get_router(state.clone()).await?;
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    // 初始化调度器
    // 启动定时任务
    tokio::spawn(start_timer(state.cancel_token.clone()));
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
