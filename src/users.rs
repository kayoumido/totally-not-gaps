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

    if let Err(e) = u {
        return Err(UserError::StudentNotFound)
    }

    let u = u.unwrap();

    if Role::from_str(&u.role).unwrap() == Role::Teacher {
        return Err(UserError::StudentNotFound)
    }

    Ok(u)
}