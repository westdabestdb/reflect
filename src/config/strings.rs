use lazy_static::lazy_static;
lazy_static! {
  pub static ref REGISTRATION_FAIL: String = "Registration failed.".to_string();
  pub static ref REGISTRATION_SUCCESS: String = "Successfully registered.".to_string();
  pub static ref EMAIL_EXIST: String = "The email is taken.".to_string();
  pub static ref USERNAME_EXIST: String = "The username is taken.".to_string();
  pub static ref SOMETHING_WRONG: String = "Something went wrong.".to_string();
  pub static ref INVALID_TOKEN: String = "Invalid token.".to_string();
}
