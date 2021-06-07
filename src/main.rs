use lazy_static::{__Deref, lazy_static};
use read_input::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use sodiumoxide::crypto::pwhash::{argon2id13, HashedPassword};

mod errors;
mod authorization;

const DATABASE_FILE: &str = "db.txt";

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Role {
    TEACHER,
    STUDENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInfo {
    username: String,
    password: String,
    role: Role,
    grades: Option<Vec<f32>>,
}

impl UserInfo {
    fn new(username: &str, password: &str, role: Role, grades: Option<Vec<f32>>) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            role,
            grades,
        }
    }
}

lazy_static! {
    static ref DATABASE: Mutex<HashMap<String, UserInfo >> = {
        let map = read_database_from_file(DATABASE_FILE).unwrap_or(HashMap::new());
        Mutex::new(map)
    };
}

fn read_database_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, UserInfo>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(user: &UserInfo) {
    match user.role {
        Role::TEACHER => teacher_action(&user),
        Role::STUDENT => student_action(&user)
    }
}

fn student_action(user: &UserInfo) {
    println!("*****\n1: See your grades\n2: About\n0: Quit");
    let choice = input().inside(0..=3).msg("Enter Your choice: ").get();
    match choice {
        1 => {
            let grades = user.grades.unwrap();
            show_grades(&grades)
        },
        2 => about(),
        0 => quit(),
        _ => panic!("impossible choice"),
    }
}

fn teacher_action(user: &UserInfo) {
    println!("*****\n1: See grades of student\n2: Enter grades\n3 About\n0: Quit");
    let choice = input().inside(0..=3).msg("Enter Your choice: ").get();
    match choice {
        1 => {
            /*
            let name: String = input().msg("Enter the name of the student which you want to see the grades: ").get();
            println!("Here are the grades of user {}", name);
            let db = DATABASE.lock().unwrap();
            match db.get(&name) {
                Some(student) => {
                    match &student.grades {
                        Some(v) => show_grades(&v),
                        None => println!("User isn't a student")
                    }
                }
                None => println!("User not in system"),
            };*/
        },
        2 => (),
        3 => about(),
        0 => quit(),
        _ => panic!("impossible choice"),
    }
}

fn show_grades(grades: &Vec<f32>) {
    println!("{:?}", grades);
    println!(
        "The average is {}",
        (grades.iter().sum::<f32>()) / ((*grades).len() as f32)
    );
}

/*
fn enter_grade() {
    println!("What is the name of the student?");
    let name: String = input().get();
    println!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    let mut map = DATABASE.lock().unwrap();
    match map.get_mut(&name) {
        Some(v) => v.push(grade),
        None => {
            ()
        }
    };
}*/

fn about() {
    panic!("The requested URL was not found on this server.");
}

fn quit() {
    println!("Saving database!");
    let file = File::create(DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, DATABASE.lock().unwrap().deref()).unwrap();
    std::process::exit(0);
}

fn login() -> UserInfo {
    loop {
        let name: String = input().msg("Enter your name: ").get();
        let pwd: String = input().msg("Enter your password: ").get();

        let db = DATABASE.lock().unwrap();

        match db.get(&name) {
            Some(v) => {
                if check_pwd(&pwd, &v.password) {
                    return v.clone();
                } else {
                    println!("{}", errors::AuthError::LoginError);
                }
            }
            None => {
                hash_pwd(&pwd);
                println!("{}", errors::AuthError::LoginError);
            }
        }
    }
}

fn check_pwd(pwd: &str, pwd_hash: &str) -> bool {
    let hp = argon2id13::HashedPassword::from_slice(pwd_hash.as_bytes()).unwrap();
    argon2id13::pwhash_verify(&hp, pwd.as_bytes())
}

fn hash_pwd(pwd: &str) -> String {
    let pwh = argon2id13::pwhash(
        pwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
        .unwrap();

    std::str::from_utf8(&pwh.0).unwrap().to_string()
}

// Feed the database with incredible data
fn init() {
    {
        let mut map = DATABASE.lock().unwrap();
        map.insert("doran".to_string(), UserInfo::new("doran", hash_pwd("guinessIsBetter").as_str(), Role::STUDENT, Some(Vec::new())));
        map.insert("bastien".to_string(), UserInfo::new("bastien", hash_pwd("farmerForThePoor").as_str(), Role::STUDENT, Some(Vec::new())));
        map.insert("alex".to_string(), UserInfo::new("alex", hash_pwd("secIsFun").as_str(), Role::TEACHER, None));
        map.insert("rene".to_string(), UserInfo::new("rene", hash_pwd("iDontLikeStudents").as_str(), Role::TEACHER, None));
    }
    println!("Saving database!");
    let file = File::create(DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, DATABASE.lock().unwrap().deref()).unwrap();
    std::process::exit(0);
}

fn main() {
    sodiumoxide::init().unwrap();
    let u = login();
    loop {
        menu(&u)
    }
}
