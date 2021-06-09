#[macro_use]
extern crate diesel;
extern crate dotenv;

use lazy_static::{__Deref, lazy_static};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};

use std::str::FromStr;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

use strum_macros::EnumString;

use crate::db::models::{User, Grade};

mod authentication;
mod authorization;
mod db;
mod errors;
mod user_input;
mod utils;
mod grades;

const DATABASE_FILE: &str = "db.txt";

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Eq, PartialEq)]
enum Role {
    #[strum(serialize = "Teacher")]
    Teacher,
    #[strum(serialize = "Student")]
    Student,
}

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(user: &User) {
    match Role::from_str(user.role.as_str()).unwrap() {
        Role::Teacher => teacher_action(&user),
        Role::Student => student_action(&user),
    }
}

fn student_action(user: &User) {
    println!("*****\n1: See your grades\n2: About\n0: Quit");
    let choice = input().inside(0..=3).msg("Enter Your choice: ").get();
    match choice {
         1 => {
             let grades = grades::get_grades(user.id);
             match grades {
                 Ok(v) => show_grades(&v),
                 Err(e) => println!("{}", e)
             }
         },
         2 => about(),
         0 => quit(),
         _ => panic!("impossible choice"),
     }
}

fn teacher_action(user: &User) {
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

fn show_grades(grades: &Vec<Grade>) {
    let sum = grades.iter().fold(0.0f32, |acc, g| acc + g.grade);
    println!("{:?}", grades);
    println!(
        "The average is {}",
        sum / ((*grades).len() as f32)
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
 * Username: bastien
 * Password: farmerForThePoor
 * Role: Student
 *
 * Username: alexandre
 * Password: laCryptoCRigolo
 * Role: Teacher
 *
 * Username: rene
 * Password: iDontLikeStudents
 * Role: Teacher
 */

fn main() {
    db::init();
    sodiumoxide::init().unwrap();

    let u = login();

    loop {
        menu(&u)
    }
    // loop {
    //     // menu(&u)
    // }
}
