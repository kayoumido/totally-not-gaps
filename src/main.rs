#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::str::FromStr;

use futures::executor::block_on;
use log::{error, info, warn};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use strum_macros::EnumString;

use crate::authorization::auth;
use crate::db::models::User;

mod authentication;
mod authorization;
mod db;
mod errors;
mod grades;
mod user_input;
mod users;
mod utils;

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
            see_grades(user, &user.username);
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
            let name: String = input()
                .msg("Enter the name of the student which you want to see the grades: ")
                .get();

            info!("{} is viewing {}' grades", user.username, name);
            see_grades(user, &name);
        }
        2 => enter_grade(),
        3 => about(),
        0 => quit(),
        _ => panic!("impossible choice"),
    }
}

fn see_grades(requester: &User, requestee: &str) {
    if !block_on(auth(&requester.username, "grades", "read")) {
        println!("Unauthorized access!");
        warn!(
            "Unauthorized access - {} tried to access {}' grades",
            requester.username, requestee
        );
        return;
    }

    // make sure that we don't have a student trying to access another student' grade
    if Role::from_str(requester.role.as_str()).unwrap() == Role::Student
        && requester.username != requestee
    {
        println!("You can not view another students grades!");

        warn!(
            "{} tried to access {}' grades with the Student role",
            requester.username, requestee
        );
    }

    println!("Here are the grades of user {}", requestee);
    match users::get_student(&requestee) {
        Ok(student) => match grades::get_grades(student.id) {
            Ok(grades) => {
                if grades.is_empty() {
                    println!("{} does not have any grades yet", requestee);
                    return;
                }

                let sum = grades.iter().fold(0.0f32, |acc, g| acc + g.grade);
                println!("{:?}", grades);
                println!("The average is {}", sum / ((*grades).len() as f32));
            }
            Err(e) => {
                warn!(
                    "{} tried to acces grades of a non-existing student or a teacher ({})",
                    requester.username, requestee
                );
                println!("{}", e)
            }
        },
        Err(e) => {
            println!("{}", e);
            warn!(
                "{} tried to acces grades of a non-existing student ({})",
                requester.username, requestee
            );
        }
    };
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
            warn!("The name of the student {} is incorrect", name);
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

    welcome();
    let u = login();

    loop {
        menu(&u)
    }
}
