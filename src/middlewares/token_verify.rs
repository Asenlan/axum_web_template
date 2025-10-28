use crate::error::AppError;

use super::{REQUEST_SHOW_PASSWD, REQUEST_SHOW_TOKEN};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::Request, middleware::Next, response::Response};
use tracing::*;
// token 验证 中间件    
pub async fn token_show_verify(req: Request, next: Next) -> Result<Response, AppError> {
    let token_str = match req.headers().get(REQUEST_SHOW_TOKEN) {
        Some(header_value) => {
            // 在这里处理获取到的 header value 的逻辑
            // 例如，转换为字符串或执行其他操作
            let token_str = header_value.to_str().unwrap_or("");
            // 这里可以使用 token_str 变量进行进一步操作
            println!("Token value: {}", token_str);
            token_str
        }
        None => {
            // 处理 header 未找到的情况
            let msg = "Token header not found".to_string();
            error!(msg);
            return Err(anyhow::anyhow!(msg).into());
        }
    };
    if verify_password(REQUEST_SHOW_PASSWD, token_str)? {
        Ok(next.run(req).await)
    } else {
        let msg = "verify password fail".to_string();
        warn!(msg);
        Err(anyhow::anyhow!(msg).into())
    }
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    // Verify password
    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}
