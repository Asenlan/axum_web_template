use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilePlatform {
    pub file_name: String,
    pub file_path: serde_json::Value,
    pub file_size: String,
    pub file_suffix: String,
    pub project_id: String,
    pub project_name: String,
    pub res_id: String,
    pub remark: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tag: String,
    pub status: String,
    pub model_id: String,
    pub exec_id: String,
}
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
impl FilePlatform {
    pub fn new(
        file_name: String,
        file_path: serde_json::Value,
        file_size: String,
        file_suffix: String,
        project_id: String,
        project_name: String,
        res_id: String,
        remark: String,
        tag: String,
        status: String,
        model_id: String,
        exec_id: String,
    ) -> Self {
        let local_now = Local::now();

        // 转换为 NaiveDateTime
        let naive_datetime: NaiveDateTime = local_now.naive_local();
        Self {
            file_name,
            file_path,
            file_size,
            file_suffix,
            project_id,
            project_name,
            res_id,
            remark,
            tag,
            status,
            model_id,
            exec_id,
            created_at: naive_datetime,
            updated_at: naive_datetime,
        }
    }
}
