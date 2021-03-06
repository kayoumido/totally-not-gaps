/*!
 * Here lays the definition of all the custom errors used in the system
 *
 * # Note
 * To simplify the definition of messages n' stuff, the crates `strum` & `strum_macros`
 * were used (thank you SEC Midterm :D)
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use std::error;
use std::fmt;
use strum::EnumMessage;
use strum_macros;

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum AuthError {
    #[strum(message = "Your login details are incorrect.")]
    LoginError,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for AuthError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}


#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum UserError {
    #[strum(message = "Student not found.")]
    StudentNotFound,
    #[strum(message = "A teacher can't have grades.")]
    TeacherCantHaveGrades,
    #[strum(message = "The try of inserting a grade has failed.")]
    FailedToInsertGrade,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for UserError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}


#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum DBError {
    #[strum(message = "Failed getting db connection")]
    ConnectionFailed,

    #[strum(message = "User not found")]
    UserNotFound,

    #[strum(message = "Failed to insert grade")]
    FailToInsertGrade,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for DBError {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}
