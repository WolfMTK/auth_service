use std::sync::Arc;
use adapter::modules::{RepositoriesModule, RepositoriesModuleExt};
use adapter::persistence::postgres::Db;
use application::usecase::user::UserUseCase;

pub struct Modules {
    user_use_case: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> Self {
        let db = Db::new().await;

        let repository_module = Arc::new(RepositoriesModule::new(db.clone()));

        let user_use_case = UserUseCase::new(repository_module.clone());

        Self {
            user_use_case
        }
    }
}
