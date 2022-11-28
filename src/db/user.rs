use crate::models::base::User;
use crate::db::Table;
use sqlx::mysql::MySqlQueryAs;

impl <'c> Table<'c, User> {
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `email`
            FROM `tbl_user`
            WHERE `id` = ?"#,
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
    }
}