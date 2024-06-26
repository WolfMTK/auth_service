use serde::{Deserialize, Serialize};
use validator::Validate;

use application::model::user::{CreateUser, UserView};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonUser {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<UserView> for JsonUser {
    fn from(value: UserView) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonCreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<JsonCreateUser> for CreateUser {
    fn from(value: JsonCreateUser) -> Self {
        CreateUser {
            username: value.username,
            email: value.email,
            password: value.password,
        }
    }
}
