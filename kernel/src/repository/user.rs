use async_trait::async_trait;

use crate::model::{
    user::{NewUser, UpdateUser, User},
    Id,
};

#[async_trait]
pub trait UserRepository {
    async fn get(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
    async fn insert(&self, source: NewUser) -> anyhow::Result<User>;
    async fn update(&self, source: UpdateUser) -> anyhow::Result<User>;
    async fn delete(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
}
