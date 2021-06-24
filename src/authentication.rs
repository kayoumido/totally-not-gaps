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

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::repository::MockPostgrSQLUserRepository;
    use crate::errors::DBError;

    #[test]
    fn test_login_with_unknown_user() {
        let mut mock = MockPostgrSQLUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Err(DBError::UserNotFound));

        let res = _login("whoamI", "password", &mock);

        assert_eq!(Err(AuthError::LoginError), res);
    }

    #[test]
    fn test_login_with_known_user_but_wrong_password() {
        let mut mock = MockPostgrSQLUserRepository::new();

        let passwd = utils::hash("password");
        mock.expect_get_user() //User::new(e, "password", "Student")
            .returning(move |_| Ok(User::new("bob", &passwd, "Teacher")));

        let res = _login("bob", "wrong", &mock);

        assert_eq!(Err(AuthError::LoginError), res);
    }

    #[test]
    fn test_login_with_known_student() {
        let mut mock = MockPostgrSQLUserRepository::new();

        let passwd = utils::hash("password");
        mock.expect_get_user() //User::new(e, "password", "Student")
            .returning(move |_| Ok(User::new("doran", &passwd, "Student")));

        let res = _login("doran", "password", &mock);

        assert_ne!(Err(AuthError::LoginError), res);

        let u = res.unwrap();

        assert_eq!("doran", u.username);
        assert_eq!("Student", u.role);
        assert!(utils::verify_hash(&u.password, "password"));
    }

    #[test]
    fn test_login_with_known_teacher() {
        let mut mock = MockPostgrSQLUserRepository::new();

        let passwd = utils::hash("password");
        mock.expect_get_user() //User::new(e, "password", "Student")
            .returning(move |_| Ok(User::new("alexandre", &passwd, "Teacher")));

        let res = _login("alexandre", "password", &mock);

        assert_ne!(Err(AuthError::LoginError), res);

        let u = res.unwrap();

        assert_eq!("alexandre", u.username);
        assert_eq!("Teacher", u.role);
        assert!(utils::verify_hash(&u.password, "password"));
    }
}
