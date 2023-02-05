#[macro_use]
extern crate diesel;

use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use diesel::QueryDsl;
use chrono::{NaiveDateTime, Local};
use models::LoginInfo;
use models::NewDoneTask;
use crate::diesel::{RunQueryDsl, ExpressionMethods};

mod task;
mod utils;
mod models;
mod schema;
use models::{NewUser, User, NewLoginInfo, NewTask, RawTask};
use task::*;
use utils::{establish_connection, hash, get_mac_address_string};
use schema::users as users_schema;
use schema::login_info as login_info_schema;
use schema::tasks as tasks_schema;
use schema::done_tasks as done_tasks_schema;


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
        point: Option<i32>,
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
                .load::<User>(&connection)?;

            let current_user = if current_user.len() > 0 {
                &current_user[0]
            } else {
                return Err(anyhow!("mismatched name or password."));
            };

            let mac_address = get_mac_address_string()?;

            let new_login_info = NewLoginInfo {
                mac_address,
                user_id: Some(current_user.id as i32),
            };

            diesel::insert_into(login_info_schema::dsl::login_info)
                .values(new_login_info)
                .execute(&connection)?;

            Ok(())
        }
        Action::Add { title, point } => {
            let connection = establish_connection();
            let mac_address = get_mac_address_string()?;
            let current_user_id = login_info_schema::dsl::login_info
                .filter(login_info_schema::dsl::mac_address.eq(&mac_address))
                .first::<LoginInfo>(&connection)?.user_id;

            let new_task = NewTask {
                title,
                user_id: current_user_id,
                point,
            };

            diesel::insert_into(tasks_schema::dsl::tasks)
                .values(new_task)
                .execute(&connection)?;

            Ok(())
        }, 
        Action::Done { id } => {
            let connection = establish_connection();

            let done_task = tasks_schema::dsl::tasks
                .filter(tasks_schema::dsl::id.eq(id as u64))
                .first::<RawTask>(&connection)?;

            let new_done_task = NewDoneTask {
                point: done_task.point,
                done_date: Local::now().naive_local(),
            };

            diesel::insert_into(done_tasks_schema::dsl::done_tasks)
                .values(new_done_task)
                .execute(&connection)?;

            diesel::delete(tasks_schema::dsl::tasks)
                .filter(tasks_schema::dsl::id.eq(id as u64))
                .execute(&connection)?;

            Ok(())
        }
        Action::Show {  } => {
            let connection = establish_connection();
            let mac_address = get_mac_address_string()?;
            let current_user_id = login_info_schema::dsl::login_info
                .filter(login_info_schema::dsl::mac_address.eq(&mac_address))
                .first::<LoginInfo>(&connection)?
                .user_id;

            let tasks = tasks_schema::dsl::tasks
                .filter(tasks_schema::dsl::user_id.eq(&current_user_id))
                .load::<RawTask>(&connection)?;

            task::Tasks::from_raw_tasks(&tasks).show_formatted();

            Ok(())
        }
    }
}
