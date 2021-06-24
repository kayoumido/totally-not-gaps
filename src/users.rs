use crate::db::models::User;
use crate::db::repository::{PostgrSQLUserRepository, UserRepository};
use crate::errors::UserError;
use crate::Role;
use std::str::FromStr;

pub fn get_student(username: &str) -> Result<User, UserError> {
    let repository = PostgrSQLUserRepository {};
    _get_student(username, &repository)
}

fn _get_student(username: &str, repository: &dyn UserRepository) -> Result<User, UserError> {
    let u = repository.get_user(username);

    if let Err(_) = u {
        return Err(UserError::StudentNotFound);
    }

    let u = u.unwrap();

    if Role::from_str(&u.role).unwrap() == Role::Teacher {
        return Err(UserError::StudentNotFound);
    }

    Ok(u)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::models::User;
    use crate::db::repository::MockPostgrSQLUserRepository;
    use crate::errors::{DBError, UserError};

    #[test]
    fn test_get_unknown_student() {
        let mut mock = MockPostgrSQLUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Err(DBError::UserNotFound));

        let res = _get_student("whoami", &mock);

        assert_eq!(Err(UserError::StudentNotFound), res);
    }

    #[test]
    fn test_get_student_with_teacher() {
        let mut mock = MockPostgrSQLUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Ok(User::new("teach", "hashedPassword", "Teacher")));

        let res = _get_student("teach", &mock);

        assert_eq!(Err(UserError::StudentNotFound), res);
    }

    #[test]
    fn test_get_student() {
        let mut mock = MockPostgrSQLUserRepository::new();

        mock.expect_get_user()
            .returning(|_| Ok(User::new("student", "hashedPassword", "Student")));

        let res = _get_student("student", &mock);

        assert_eq!(Ok(User::new("student", "hashedPassword", "Student")), res);
    }
}
