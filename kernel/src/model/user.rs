use crate::model::{hash_password, Id};

pub struct User {
    pub id: Id<User>,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct NewUser {
    pub id: Id<User>,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(id: Id<User>, username: String, email: String, mut password: String) -> Self {
        password = hash_password(password);

        Self {
            id,
            username,
            email,
            password,
        }
    }
}

pub struct UpdateUser {
    pub id: Id<User>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UpdateUser {
    pub fn new(
        id: Id<User>,
        username: Option<String>,
        email: Option<String>,
        mut password: Option<String>,
    ) -> Self {
        if password.is_some() {
            password = Option::from(hash_password(password.unwrap().to_string()))
        }

        Self {
            id,
            username,
            email,
            password,
        }
    }
}
