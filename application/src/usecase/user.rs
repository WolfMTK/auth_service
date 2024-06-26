use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use kernel::repository::user::UserRepository;

use crate::model::user::{CreateUser, UserView};

pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self { Self { repositories } }

    pub async fn create_user(&self, source: CreateUser) -> anyhow::Result<UserView> {
        let user_view = self
            .repositories
            .user_repository()
            .insert(source.try_into()?)
            .await?;

        Ok(user_view.into())
    }
}
