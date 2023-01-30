use secrecy::{ExposeSecret, Secret, SecretString};
use validator::{validate_length, Validate, ValidationError};

pub fn password_validator(password: &SecretString) -> Result<(), ValidationError> {
    let len = password.expose_secret().len();

    match len {
        len if len <= 8 => Err(ValidationError::new("Password too short!")),
        len if len >= 32 => Err(ValidationError::new("Password too long!")),
        _ => Ok(()),
    }
}
