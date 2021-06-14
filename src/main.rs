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
use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;

mod authentication;
mod authorization;
mod db;
mod errors;
mod user_input;
mod utils;
mod grades;
mod users;

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
            info!("{} tried to see his grades", user.username);
            let grades = grades::get_grades(user.id);
            match grades {
                Ok(v) => show_grades(&v),
                Err(e) => {
                    println!("{}", e);
                    error!("An error has occured")
                }
            }
        }
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
            let name: String = input().msg("Enter the name of the student which you want to see the grades: ").get();
            println!("Here are the grades of user {}", name);
            let u = users::get_student(&name);
            match u {
                Ok(student) => {
                    let grades = grades::get_grades(student.id);
                    match grades {
                        Ok(v) => show_grades(&v),
                        Err(e) => println!("{}", e)
                    }
                }
                Err(e) => println!("{}", e),
            };
        }
        2 => enter_grade(),
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

fn enter_grade() {
    println!("What is the name of the student?");
    let name: String = input().get();
    let u = users::get_student(&name);

    match u {
        Ok(student) => {
            println!("What is the new grade of the student?");
            let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
            let result_insertion = grades::insert_grade(student.id, grade);
            if let Err(e) = result_insertion {
                println!("{}", e);
                error!("The insertion of the new student has failed");
            }
        }
        Err(e) => {
            println!("{}", e);
            warn!("The name of the student {} is incorrect");
        }
    }
}

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

            warn!("{} tried to log in", email);

            continue;
        }

        info!("{} has successfully logged in", email);

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
    SimpleLogger::new().init().unwrap();
    sodiumoxide::init().unwrap();

    let u = login();

    loop {
        menu(&u)
    }
}
