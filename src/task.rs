use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use chrono::{Local, DateTime};
use std::fmt;
use crate::models::RawTask;
use crate::schema::tasks::point;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u64,
    title: String,
    point: Option<i32>
}

impl Task {
    pub fn from_raw_task(raw_task: &RawTask) -> Self {
        Self {
            id: raw_task.id,
            title: raw_task.title.to_owned(),
            point: raw_task.point
        }
    }
}


pub struct Tasks {
    tasks: Vec<Task>
}

impl Tasks {
    pub fn from_raw_tasks(raw_tasks: &Vec<RawTask>) -> Self {
        Self {
            tasks: 
                raw_tasks
                    .iter()
                    .map(
                        |raw_task: &RawTask| 
                        Task::from_raw_task(raw_task)
                    )
                    .collect()
        }
    }

    pub fn show_formatted(self) {
        println!("{:^3}|{:^50}|{:^5}", "id", "title", "points");
        for task in self.tasks {
            if let Some(points) = task.point {
                println!("{:>3}|{:<50}|{:>5}", task.id, task.title, points);
            } else {
                println!("{:>3}|{:<50}|{:>5}", task.id, task.title, "-");
            }
        };
    }
}
