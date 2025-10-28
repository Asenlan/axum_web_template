use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllResult {
    pub status: u64,
    pub content: String,
    pub msg: String,
}
#[allow(dead_code)]
//构造返回值
impl AllResult {
    pub fn new(status: u64, content: String, msg: String) -> Self {
        Self {
            status,
            content,
            msg,
        }
    }
    pub fn success() -> Self {
        Self {
            status: 1,
            content: "".to_string(),
            msg: "execute success".to_string(),
        }
    }

    pub fn success_content(content: String) -> Self {
        Self {
            status: 1,
            content,
            msg: "execute success".to_string(),
        }
    }
}
