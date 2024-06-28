use serde::Deserialize;

use crate::model::validate_email;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    pub fn try_new(email: String) -> Result<Self, String> {
        if validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("invalid email {}", email))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn inner(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
