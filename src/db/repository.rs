use super::connection;
use super::models::*;
use super::schema::users::dsl::*;
use super::schema::grades::dsl::{student_id, grades};

use crate::errors::DBError;

use diesel::{insert_into, prelude::*};

pub trait UserRepository {
    /// Try and get a user from the storage
    /// if the wanted user doesn't exist, an error is returned
    ///
    /// # Arguments
    ///
    /// * `uname` - username of the user
    ///
    fn get_user(&self, uname: &str) -> Result<User, DBError>;

    /// Try and get a user from the storage
    /// if the wanted user doesn't exist, an error is returned
    ///
    /// # Arguments
    ///
    /// * `user_id` - id of the user
    ///
    fn get_user_by_id(&self, user_id: i32) -> Result<User, DBError>;

    /// Try and get the grades of the user from the storage
    /// If the user doesn't exist, an error is returned
    ///
    /// # Arguments
    ///
    /// * `user_id` - id of the user
    fn get_grades(&self, user_id: i32) -> Result<Vec<Grade>, DBError>;
}

pub struct PostgrSQLUserRepository {}

/// Implementation of the `UserRepository` with PostgreSQL as a storage
impl UserRepository for PostgrSQLUserRepository {
    fn get_user(&self, usrname: &str) -> Result<User, DBError> {
        let conn = connection()?;

        let res = users.filter(username.eq(usrname)).first::<User>(&conn);

        if let Err(_) = res {
            Err(DBError::UserNotFound)
        } else {
            Ok(res.unwrap())
        }
    }

    fn get_user_by_id(&self, user_id: i32) -> Result<User, DBError> {
        let conn = connection()?;

        let res = users.filter(id.eq(user_id)).first::<User>(&conn);

        if let Err(_) = res {
            Err(DBError::UserNotFound)
        } else {
            Ok(res.unwrap())
        }
    }

    fn get_grades(&self, user_id: i32) -> Result<Vec<Grade>, DBError> {
        let conn = connection()?;

        let res = grades.filter(student_id.eq(user_id)).load::<Grade>(&conn);

        if let Err(_) = res {
            Err(DBError::UserNotFound)
        } else {
            Ok(res.unwrap())
        }
    }
}
