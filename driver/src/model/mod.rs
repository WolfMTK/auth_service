use regex::Regex;

pub mod email;
pub mod paginate;
pub mod user;

const EMAIL_PATTERN: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

pub fn validate_email(email: &str) -> bool {
    let pattern = Regex::new(EMAIL_PATTERN).unwrap();

    pattern.is_match(email)
}
