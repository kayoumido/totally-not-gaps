use super::schema::{grades, users};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
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
