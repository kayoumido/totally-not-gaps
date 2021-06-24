use std::str::FromStr;

use crate::db::models::Grade;
use crate::db::repository::{
    GradeRepository, PostgrSQLGradeRepository, PostgrSQLUserRepository, UserRepository,
};
use crate::errors::UserError;
use crate::Role;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::models::{Grade, User};
    use crate::db::repository::{MockPostgrSQLGradeRepository, MockPostgrSQLUserRepository};
    use crate::errors::{DBError, UserError};
    use crate::utils;

    #[test]
    fn test_failed_grade_insert() {
        let mut mock = MockPostgrSQLGradeRepository::new();

        mock.expect_insert_grade()
            .returning(|_, _| Err(DBError::FailToInsertGrade));

        let res = _insert_grade(5, 6.0, &mock);

        assert_eq!(Err(UserError::FailedToInsertGrade), res);
    }

    #[test]
    fn test_succesfull_grade_insert() {
        let mut mock = MockPostgrSQLGradeRepository::new();

        mock.expect_insert_grade().returning(|_, _| Ok(()));

        let res = _insert_grade(1, 6.0, &mock);

        assert_eq!(Ok(()), res);
    }

    #[test]
    fn test_get_grade_with_unknown_user() {
        let mut mock = MockPostgrSQLUserRepository::new();

        mock.expect_get_user_by_id()
            .returning(|_| Err(DBError::UserNotFound));

        let res = _get_grades(1, &mock);

        assert_eq!(Err(UserError::StudentNotFound), res);
    }

    #[test]
    fn test_get_grade_with_teacher() {
        let mut mock = MockPostgrSQLUserRepository::new();

        let passwd = utils::hash("password");
        mock.expect_get_user_by_id()
            .returning(move |_| Ok(User::new("bob", &passwd, "Teacher")));

        let res = _get_grades(1, &mock);

        assert_eq!(Err(UserError::TeacherCantHaveGrades), res);
    }

    #[test]
    fn test_get_grade_with_student_without_grades() {
        let mut mock = MockPostgrSQLUserRepository::new();

        let passwd = utils::hash("password");
        mock.expect_get_user_by_id()
            .returning(move |_| Ok(User::new("bob", &passwd, "Student")));

        mock.expect_get_grades()
            .returning(|_| Err(DBError::UserNotFound));

        let res = _get_grades(1, &mock);

        assert_eq!(Ok(Vec::new()), res);
    }

    #[test]
    fn test_get_grade_with_student_with_grades() {
        let mut mock = MockPostgrSQLUserRepository::new();

        let passwd = utils::hash("password");
        mock.expect_get_user_by_id()
            .returning(move |_| Ok(User::new("bob", &passwd, "Student")));

        mock.expect_get_grades().returning(|_| {
            Ok(vec![
                Grade {
                    id: 1,
                    grade: 4.5,
                    student_id: 1,
                },
                Grade {
                    id: 2,
                    grade: 5.1,
                    student_id: 1,
                },
            ])
        });

        let res = _get_grades(1, &mock);

        assert_eq!(
            Ok(vec![
                Grade {
                    id: 1,
                    grade: 4.5,
                    student_id: 1,
                },
                Grade {
                    id: 2,
                    grade: 5.1,
                    student_id: 1,
                },
            ]),
            res
        );
    }
}
