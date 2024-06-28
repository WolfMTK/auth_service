use async_trait::async_trait;
use sqlx::{query, query_as};

use kernel::model::{
    Id,
    user::{NewUser, UpdateUser, User},
};
use kernel::repository::user::UserRepository;

use crate::model::user::{InsertUser, StoredUser, UpdateStoredUser};
use crate::repository::DatabaseRepositoryImpl;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn get(&self, id: &Id<User>) -> anyhow::Result<Option<User>> {
        let pool = self.db.0.clone();
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
            .bind(id.value.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_user {
            Some(val) => Ok(Some(val.try_into()?)),
            None => Ok(None),
        }
    }

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

    async fn update(&self, source: UpdateUser) -> anyhow::Result<User> {
        let pool = self.db.0.clone();
        let user: UpdateStoredUser = source.into();
        let id = user.id.clone();
        let update_sql = r#"
            update
                users as target
            set
                username = case when $2 is not null then $2 else target.username end,
                email = case when $3 is not null then $3 else target.email end,
                password = case when $4 is not null then $4 else target.password end
            where
                target.id = $1
        "#;
        let _ = query(update_sql)
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
