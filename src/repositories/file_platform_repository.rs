use crate::{models::file_platform_model::FilePlatform, AppState};
use anyhow::Error;

#[allow(dead_code)]
impl AppState {
    // 创建执行结果,返回执行结果id
    pub async fn create_res(&self, file: FilePlatform) -> Result<String, Error> {
        let todo_id = sqlx::query!(
            r#"INSERT INTO fa_resource (file_name,file_path,file_size, file_suffix,project_id,project_name,res_id,remark,tag,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?) "#,
            &file.file_name,
            &file.file_path,
            &file.file_size,
            &file.file_suffix,
            &file.project_id,
            &file.project_name,
            &file.res_id,
            &file.remark,
            &file.tag,
            &file.created_at,
            &file.updated_at,
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();
        Ok(todo_id.to_string())
    }
    // 更新执行结果,返回执行结果id 
    pub async fn update_exec(&self, file: FilePlatform) -> Result<String, Error> {
        let todo_id=sqlx::query!(
            r#"UPDATE fa_model_task  SET res_id = ?, res_path = ?, updated_at =? WHERE exec_id = ?"#,
            &file.res_id,
            &file.file_path,
            &file.updated_at,
            &file.exec_id,
        )
        .execute(&self.pool)
        .await?.last_insert_id();
        Ok(todo_id.to_string())
    }
}
