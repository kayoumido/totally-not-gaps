#[macro_use]
extern crate diesel;
extern crate dotenv;

use lazy_static::{__Deref, lazy_static};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

use crate::db::models::User;

mod authentication;
mod authorization;
mod db;
mod errors;
mod user_input;
mod utils;

const DATABASE_FILE: &str = "db.txt";

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Role {
    Teacher,
    Student,
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
    static ref DATABASE: Mutex<HashMap<String, UserInfo>> = {
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
        Role::Teacher => teacher_action(&user),
        Role::Student => student_action(&user),
    }
}

fn student_action(user: &UserInfo) {
    // println!("*****\n1: See your grades\n2: About\n0: Quit");
    // let choice = input().inside(0..=3).msg("Enter Your choice: ").get();
    // match choice {
    //     1 => {
    //         let grades = user.grades.unwrap();
    //         show_grades(&grades)
    //     }
    //     2 => about(),
    //     0 => quit(),
    //     _ => panic!("impossible choice"),
    // }
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
        }
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

pub fn login() -> User {
    println!("\nLogin:");

    loop {
        let email = user_input::ask_for_email();
        let passwd = user_input::ask_for_password();

        let u = authentication::login(&email, &passwd);
        if let Err(e) = u {
            println!("{}", e);
            continue;
        }

        return u.unwrap();
    }
}

/**
 * Available users:
 * Username: doran
 * Password: guinessIsBetter
 * Role: Student
 *
 * Username: bastion
 * Password: farmerForThePoor
 * Role: Student
 *
 * Username: alexandre
 * Password: laCryptoCRigolo
 * Role: Teacher
 *
 * Username: rene
 * Password: guinessIsBetter
 * Role: Teacher
 */

fn main() {
    db::init();
    sodiumoxide::init().unwrap();

    let u = login();
    println!("{:?}", u);
    // loop {
    //     // menu(&u)
    // }
}
