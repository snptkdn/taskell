#[macro_use]
extern crate diesel;

use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use diesel::QueryDsl;
use crate::diesel::{RunQueryDsl, ExpressionMethods};
use std::fs::OpenOptions;
use std::io::prelude::*;

mod task;
mod utils;
mod models;
mod schema;
use models::{NewUser, User};
use task::*;
use utils::{establish_connection, hash};
use schema::users as users_schema;


#[derive(Parser)]
#[clap(
    name = "Taskell",
    author = "snptkdn",
    version = "v0.0.1",
    about = "Taskell is task management tool."
)]
struct AppArg {
    #[clap(subcommand)]
    action: Action
}

#[derive(Subcommand)]
enum Action {
    SignUp {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        pass: String,
    },
    Login {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        pass: String,
    },
    Add {
        title: String,

        #[clap(short, long)]
        point: Option<usize>,
    },

    Done {
        id: usize,
    },

    Show {}
}

fn main() -> Result<()> {
    let cli = AppArg::parse();
    match cli.action {
        Action::SignUp { name, pass } => {
            let connection = establish_connection();
            let new_user = NewUser {
                name,
                encrypted_pass: hash(pass)
            };

            let same_name_user = users_schema::dsl::users
                .filter(users_schema::name.eq(&new_user.name))
                .load::<User>(&connection)?;

            if same_name_user.len() > 0 {
                return Err(anyhow!("this name is already exists."))
            }

            diesel::insert_into(users_schema::dsl::users)
                .values(new_user)
                .execute(&connection)?;

            Ok(())
        },
        Action::Login { name, pass } => {
            let connection = establish_connection();
            let current_user = &users_schema::dsl::users
                .filter(users_schema::dsl::name.eq(&name))
                .filter(users_schema::dsl::encrypted_pass.eq(hash(pass)))
                .load::<User>(&connection)?[0];

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open("user_info.json")?;

            file.write_all(current_user.id.to_string().as_bytes())?;
            file.flush()?;

            Ok(())
        }
        Action::Add { title, point } => {
            let task = Task::new(title, point);
            let mut tasks = load_task()?;
            tasks.insert
            (
                if let Some(x) = tasks.keys().max() {
                    x+1
                } else {
                    1
                }, 
                task
            );
            write_file(tasks).unwrap();
            Ok(())
        }, 
        Action::Done { id } => {
            let tasks = load_task()?;
            let tasks = delete_task(tasks, id)?;
            write_file(tasks)?;
            Ok(())
        }
        Action::Show {  } => {
            println!("|{:^3}|{:^50}|{:^5}|", "id", "title", "point");
            for (i, task) in load_task()? {
                println!("|{:>3}|{}|", i, task)
            }
            Ok(())
        }
    }
}
