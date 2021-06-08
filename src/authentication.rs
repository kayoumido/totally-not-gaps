use crate::db::models::User;
use crate::db::repository::{PostgrSQLUserRepository, UserRepository};
use crate::errors::AuthError;
use crate::utils;

/// Public function for the login
/// See `_login` for more info
///
pub fn login(username: &str, passwd: &str) -> Result<User, AuthError> {
    let repository = PostgrSQLUserRepository {};
    _login(username, passwd, &repository)
}

/// User login
///
/// # Arguments
///
/// * `email` - the email of the user trying to login
///
/// * `passwd` - the password of the user trying to login
///
/// * `repository` - the user repository to interact with
///
fn _login(
    username: &str,
    passwd: &str,
    repository: &dyn UserRepository,
) -> Result<User, AuthError> {
    // get all the user info we need from the database
    let u = repository.get_user(username);

    if let Err(_) = u {
        // to avoid timing attacks, perform a argon2 hash to "waste" time
        utils::hash(passwd);
        return Err(AuthError::LoginError);
    }

    let u = u.unwrap();
    // check the password
    if utils::verify_hash(&u.password, passwd) {
        Ok(u)
    } else {
        Err(AuthError::LoginError)
    }
}
