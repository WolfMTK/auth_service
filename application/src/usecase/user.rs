use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use kernel::{model::user::UpdateUser, repository::user::UserRepository};

use crate::model::user::{CreateUser, UpdateUserView, UserView};

pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn create_user(&self, source: CreateUser) -> anyhow::Result<UserView> {
        let user_view = self
            .repositories
            .user_repository()
            .insert(source.try_into()?)
            .await?;

        Ok(user_view.into())
    }

    pub async fn update_user(&self, source: UpdateUserView) -> anyhow::Result<UserView> {
        let update_user = UpdateUser::new(
            source.id.try_into()?,
            source.username,
            source.email,
            source.password,
        );

        let user_view = self
            .repositories
            .user_repository()
            .update(update_user)
            .await?;

        Ok(user_view.into())
    }
}
