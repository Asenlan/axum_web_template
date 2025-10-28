use crate::{error::AppError, AppConfig};
use anyhow::Result;
use axum::{body::to_bytes, extract::Request, middleware::Next, response::Response};
use serde_json::{from_slice, to_string, Value};
use std::path::PathBuf;
// 路径中间件: 用于处理请求体中的 import_name 字段 并将其替换为完整的路径
pub async fn path_middleware(req: Request, next: Next) -> Result<Response, AppError> {
    // 获取配置（需要提前注入到扩展中）
    let app_config = AppConfig::load()?;

    // 处理请求体
    let (parts, body) = req.into_parts();

    // 转换 body 为 Bytes
    let bytes = to_bytes(body, usize::MAX).await?;

    let modified_body = if let Ok(mut body_json) = from_slice::<Value>(&bytes) {
        // 根据环境状态判断是否需要处理路径
        if app_config.server.status == "DEV" {
            process_import_name(
                &mut body_json,
                &app_config.server.import_dir.to_string_lossy(),
            );
        }
        to_string(&body_json)?.into_bytes()
    } else {
        bytes.to_vec()
    };

    // 重新构建请求
    let req = Request::from_parts(parts, axum::body::Body::from(modified_body));
    Ok(next.run(req).await)
}

// 递归处理 JSON 结构
fn process_import_name(value: &mut Value, base_path: &str) {
    match value {
        Value::Object(map) => {
            if let Some(import_name) = map.get_mut("import_name") {
                if let Some(s) = import_name.as_str() {
                    if !s.is_empty() {
                        let full_path = PathBuf::from(base_path)
                            .join(s)
                            .to_string_lossy()
                            .into_owned();
                        *import_name = Value::String(full_path);
                    }
                }
            }
            // 递归处理嵌套结构
            for (_, v) in map.iter_mut() {
                process_import_name(v, base_path);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                process_import_name(item, base_path);
            }
        }
        _ => {}
    }
}
