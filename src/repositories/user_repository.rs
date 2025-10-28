use crate::{models::user_model::User, AppState};
use anyhow::Error;

impl AppState {
    // 创建用户
    pub async fn create_user(&self, _user: User) -> Result<String, Error> {
        // let todo_id = sqlx::query!(
        //     r#"INSERT INTO user (name, email, password, age) VALUES (?, ?, ?, ?) "#,
        //     &user.name,
        //     &user.email,
        //     &user.password,
        //     &user.age
        // )
        // .execute(&self.pool)
        // .await?
        // .last_insert_id();
        // Ok(todo_id.to_string())
        todo!()
    }

    // 获取用户
    pub async fn check_user(&self, id: i32) -> Result<User, Error> {
        let user: User = sqlx::query_as(r#"select * from casbin_rule where id = ?"#)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
    // 显示所有用户
    pub async fn show_user(&self) -> Result<Vec<User>, Error> {
        let users: Vec<User> = sqlx::query_as(r#"select * from casbin_rule"#)
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
}
