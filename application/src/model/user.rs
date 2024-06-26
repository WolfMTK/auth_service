use kernel::model::Id;
use kernel::model::user::{NewUser, User};

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
        Ok(NewUser::new(Id::gen(), value.username, value.email, value.password))
    }
}
