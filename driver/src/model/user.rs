use serde::{Deserialize, Serialize};
use validator::Validate;

use application::model::user::{CreateUser, UpdateUserView, UserView};

use crate::model::email::Email;

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

#[derive(Debug, Serialize)]
pub struct JsonUserId {
    pub id: String,
}

impl From<UserView> for JsonUserId {
    fn from(value: UserView) -> Self {
        Self { id: value.id }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonCreateUser {
    pub username: String,
    pub email: Email,
    pub password: String,
}

impl From<JsonCreateUser> for CreateUser {
    fn from(value: JsonCreateUser) -> Self {
        CreateUser {
            username: value.username,
            email: value.email.into_inner(),
            password: value.password,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonUpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl JsonUpdateUser {
    pub fn to_view(self, id: String) -> UpdateUserView {
        UpdateUserView::new(id, self.username, self.email, self.password)
    }
}
