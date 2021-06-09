use crate::db::models::Grade;
use crate::db::repository::{PostgrSQLUserRepository, UserRepository};
use crate::errors::UserError;
use std::str::FromStr;
use crate::Role;

pub fn get_grades(user_id: i32) -> Result<Vec<Grade>, UserError> {
    let repository = PostgrSQLUserRepository {};
    _get_grades(user_id, &repository)
}

fn _get_grades(user_id: i32, repository: &dyn UserRepository) -> Result<Vec<Grade>, UserError> {

    let u = repository.get_user_by_id(user_id);

    if let Err(e) = u {
        return Err(UserError::StudentNotFound);
    }

    let u = u.unwrap();

    if Role::from_str(&u.role).unwrap() == Role::Teacher {
        return Err(UserError::TeacherCantHaveGrades);
    }

    match repository.get_grades(user_id) {
        Ok(v) => Ok(v),
        Err(e) => Ok(Vec::new())
    }
}

