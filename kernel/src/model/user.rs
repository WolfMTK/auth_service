use crate::model::Id;

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
    pub fn new(id: Id<User>, username: String, email: String, password: String) -> Self {
        Self {
            id,
            username,
            email,
            password,
        }
    }
}
