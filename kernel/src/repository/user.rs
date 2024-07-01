use async_trait::async_trait;

use crate::model::{
    Id,
    paginate::LimitAndQuery,
    user::{NewUser, UpdateUser, User},
};

#[async_trait]
pub trait UserRepository {
    async fn get(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
    async fn check_user(&self, email: String, username: String) -> anyhow::Result<bool>;
    async fn get_all(&self, limit_and_query: LimitAndQuery) -> anyhow::Result<Option<Vec<User>>>;
    async fn insert(&self, source: NewUser) -> anyhow::Result<User>;
    async fn update(&self, source: UpdateUser) -> anyhow::Result<User>;
    async fn delete(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
}
