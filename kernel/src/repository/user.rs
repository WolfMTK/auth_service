use async_trait::async_trait;

use crate::model::{
    Id,
    user::{NewUser, UpdateUser, User},
};

#[async_trait]
pub trait UserRepository {
    async fn get(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
    async fn insert(&self, source: NewUser) -> anyhow::Result<User>;
    async fn update(&self, source: UpdateUser) -> anyhow::Result<User>;
}
