use crate::db::models::Grade;
use crate::db::repository::{
    GradeRepository, PostgrSQLGradeRepository, PostgrSQLUserRepository, UserRepository,
};
use crate::errors::UserError;
use crate::Role;
use std::str::FromStr;

pub fn insert_grade(user_id: i32, grade: f32) -> Result<(), UserError> {
    let repository = PostgrSQLGradeRepository {};
    _insert_grade(user_id, grade, &repository)
}

pub fn get_grades(user_id: i32) -> Result<Vec<Grade>, UserError> {
    let repository = PostgrSQLUserRepository {};
    _get_grades(user_id, &repository)
}

fn _insert_grade(
    user_id: i32,
    grade: f32,
    repository: &dyn GradeRepository,
) -> Result<(), UserError> {
    let r = repository.insert_grade(user_id, grade);

    if let Err(_) = r {
        return Err(UserError::FailedToInsertGrade);
    }

    Ok(())
}

fn _get_grades(user_id: i32, repository: &dyn UserRepository) -> Result<Vec<Grade>, UserError> {
    let u = repository.get_user_by_id(user_id);

    if let Err(_) = u {
        return Err(UserError::StudentNotFound);
    }

    let u = u.unwrap();

    if Role::from_str(&u.role).unwrap() == Role::Teacher {
        return Err(UserError::TeacherCantHaveGrades);
    }

    match repository.get_grades(user_id) {
        Ok(v) => Ok(v),
        Err(_) => Ok(Vec::new()),
    }
}
