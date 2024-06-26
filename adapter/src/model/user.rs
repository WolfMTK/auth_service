use sqlx::FromRow;
use kernel::model::user::{NewUser, User};

#[derive(FromRow, Debug)]
pub struct StoredUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String
}

impl TryFrom<StoredUser> for User {
    type Error = anyhow::Error;

    fn try_from(value: StoredUser) -> Result<Self, Self::Error> {
        Ok(User {
            id: value.id.try_into()?,
            username: value.username,
            email: value.email,
            password: value.password
        })
    }
}

#[derive(FromRow, Debug)]
pub struct InsertUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String
}

impl From<NewUser> for InsertUser {
    fn from(value: NewUser) -> Self {
        InsertUser {
            id: value.id.value.to_string(),
            username: value.username,
            email: value.email,
            password: value.password
        }
    }
}
