use super::schema::{grades, users};

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

impl User {
    /// Only exists for the unit tests
    pub fn new(username: &str, passwd: &str, role: &str) -> Self {
        Self {
            id: 1,
            username: username.to_string(),
            password: passwd.to_string(),
            role: role.to_string(),
        }
    }
}

#[derive(Queryable, Debug, Associations)]
#[belongs_to(User foreign_key = "student_id")]
pub struct Grade {
    pub id: i32,
    pub grade: f32,
    pub student_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub role: &'a str,
}

#[derive(Insertable)]
#[table_name = "grades"]
pub struct NewGrade {
    pub grade: f32,
    pub student_id: i32,
}
