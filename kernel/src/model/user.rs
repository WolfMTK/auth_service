use crate::model::Id;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

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
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        password = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Self {
            id,
            username,
            email,
            password,
        }
    }
}
