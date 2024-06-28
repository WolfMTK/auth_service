use kernel::model::user::{NewUser, User};
use kernel::model::Id;

pub struct UserView {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for UserView {
    fn from(value: User) -> Self {
        Self {
            id: value.id.value.to_string(),
            username: value.username,
            email: value.email,
        }
    }
}

pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl CreateUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

impl TryFrom<CreateUser> for NewUser {
    type Error = anyhow::Error;

    fn try_from(value: CreateUser) -> Result<Self, Self::Error> {
        Ok(NewUser::new(
            Id::gen(),
            value.username,
            value.email,
            value.password,
        ))
    }
}

pub struct UpdateUserView {
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserView {
    pub fn new(
        id: String,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password,
        }
    }
}
