use async_trait::async_trait;
use sqlx::{query, query_as};

use kernel::model::user::{NewUser, User};
use kernel::repository::user::UserRepository;

use crate::model::user::{InsertUser, StoredUser};
use crate::repository::DatabaseRepositoryImpl;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn insert(&self, source: NewUser) -> anyhow::Result<User> {
        let pool = self.db.0.clone();
        let user: InsertUser = source.into();
        let id = user.id.clone();

        let insert_sql = r#"
            insert into
                users (id, username, email, password)
            values
                ($1, $2, $3, $4)
        "#;

        let _ = query(insert_sql)
            .bind(user.id)
            .bind(user.username)
            .bind(user.email)
            .bind(user.password)
            .execute(&*pool)
            .await?;

        let get_sql = r#"
            select
                u.id as id,
                u.username as username,
                u.email as email,
                u.password as password
            from
                users as u
            where
                u.id = $1
        "#;

        let stored_user = query_as::<_, StoredUser>(get_sql)
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Ok(stored_user.try_into()?)
    }
}
