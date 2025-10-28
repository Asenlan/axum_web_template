mod config;
mod error;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod scheduler;
mod services;
mod util;
use error::AppError;
use tokio_util::sync::CancellationToken;

use std::{fmt, ops::Deref, sync::Arc};

use anyhow::Context;
pub use config::AppConfig;
pub use routes::get_router;
pub use scheduler::task::start_timer;

use sqlx::MySqlPool;
use tokio::fs;
use util::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: MySqlPool,
    // 定时任务取消令牌
    pub cancel_token: CancellationToken,
}

// 当我调用 state.config => state.inner.config
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("create base_dir failed")?;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let pool = MySqlPool::connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
                cancel_token: CancellationToken::new(),
            }),
        })
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
