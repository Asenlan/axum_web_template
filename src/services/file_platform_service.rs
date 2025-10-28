use anyhow::Error;
use serde_json::Value;
use tracing::info;
use uuid::Uuid;

use crate::{models::file_platform_model::FilePlatform, AppState};

// 设置存储文件到数据库
pub async fn set_file(
    state: AppState,
    file_name: &str,
    output_file: &str,
    project_id: &str,
    exec_id: &str,
) -> Result<String, Error> {
    // [{"tag": "jpg", "url": "uploads/file/project/f458cd32-632b-44dd-a88f-ef66cd6686a3/11.jpg", "name": "pexels-jonathanborba-14832165.jpg", "size": "0.82M"}]
    let json_str = format!(
        r#"[{{"tag": "csv","url": "{}", "name": "{}"}}]"#,
        output_file, file_name
    );
    info!("json_str: {}", json_str);
    // 使用serde_json的from_str函数将字符串转换为JSON数据结构
    let json_value: Value = serde_json::from_str(&json_str).expect("Invalid JSON string");
    let fa: FilePlatform = FilePlatform::new(
        file_name.to_string(),
        json_value,
        "".to_string(),
        "csv".to_string(),
        project_id.to_string(),
        "".to_string(),
        Uuid::now_v7().to_string(),
        "".to_string(),
        "result".to_string(),
        "".to_string(),
        "".to_string(),
        exec_id.to_string(),
    );
    let todo_id = state.create_res(fa.clone()).await?;
    info!("数据插入成功id: {}", todo_id);
    if !exec_id.is_empty() {
        state.update_exec(fa.clone()).await?;
    }
    info!("结果文件更新成功，id: {}", fa.exec_id);
    Ok(todo_id)
}

// 只更新文件不创建新数据
pub async fn update_file(
    state: AppState,
    file_name: &str,
    output_file: &str,
    project_id: &str,
    exec_id: &str,
) -> Result<String, Error> {
    let json_str = format!(
        r#"[{{"tag": "csv","url": "{}", "name": "{}"}}]"#,
        output_file, file_name
    );
    info!("json_str: {}", json_str);
    // 使用serde_json的from_str函数将字符串转换为JSON数据结构
    let json_value: Value = serde_json::from_str(&json_str).expect("Invalid JSON string");
    let fa: FilePlatform = FilePlatform::new(
        file_name.to_string(),
        json_value,
        "".to_string(),
        "csv".to_string(),
        project_id.to_string(),
        "".to_string(),
        Uuid::now_v7().to_string(),
        "".to_string(),
        "result".to_string(),
        "".to_string(),
        "".to_string(),
        exec_id.to_string(),
    );
    state.update_exec(fa.clone()).await?;
    info!("结果文件更新成功，id: {}", fa.exec_id);
    Ok(fa.res_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppConfig;
    use anyhow::Result;
    use serde_json::Value;
    #[tokio::test]
    async fn create_res_test() -> Result<()> {
        let app_config = AppConfig::load()?;
        let appstate = AppState::try_new(app_config).await?;
        // let column_names = "";
        // 包含JSON字符串的Rust字符串
        let json_str = r#"[{"url": "uploads/file/project/ae5d08fa-413b-4a8e-a313-3be8a8108a20/result/dc64a36a_6e24_4a62_8a1e_398fe26760ce.csv", "name": "一级上分缺失数据1", "type": "csv"}]"#;

        // 使用serde_json的from_str函数将字符串转换为JSON数据结构
        let json_value: Value = serde_json::from_str(json_str).expect("Invalid JSON string");
        let fa: FilePlatform = FilePlatform::new(
            "file_name".to_string(),
            json_value,
            "file_size".to_string(),
            "file_suffix".to_string(),
            "3000".to_string(),
            "project_name".to_string(),
            "res_id".to_string(),
            "remark".to_string(),
            "tag".to_string(),
            "status".to_string(),
            "fede2e2c-73d1-413b-a056-589a59390515".to_string(),
            "4b56d18c-738c-47ac-8a05-41b51fb5e31e".to_string(),
            //
        );
        let a = appstate.create_res(fa.clone()).await?;
        println!("{}", a);
        let b = appstate.update_exec(fa).await?;
        println!("{}", b);
        Ok(())
    }
}
