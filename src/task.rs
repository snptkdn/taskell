use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f32::consts::E;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    title: String,
    point: Option<usize>
}

impl Task {
    pub fn new(title: String, point: Option<usize>) -> Self {
        Task {
            title, 
            point
        }
    }
}

use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(point) = self.point {
            write!(f, "{}|{}", self.title, point)
        } else {
            write!(f, "{}", self.title)
        }
    }
}

pub fn delete_task(mut tasks: HashMap<usize, Task>, id: usize) -> Result<HashMap<usize, Task>> {
    println!("before delete {:?}", &tasks);
    tasks.remove(&id);
    println!("after delete {:?}", &tasks);
    Ok(tasks)
}

pub fn load_task() -> Result<HashMap<usize, Task>> {
    let tasks_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("task.json")?;
    let reader = BufReader::new(tasks_file);
    let tasks: HashMap<usize, Task> = 
        match serde_json::from_reader(reader) {
            Ok(tasks) => tasks,
            Err(e) => HashMap::new()
        };
    Ok(tasks)
}

pub fn write_file(tasks: HashMap<usize, Task>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("task.json")?;

    let memos_text = serde_json::to_string(&tasks)?;
    println!("{:?}", tasks);
    //write!(&file, "{}", memos_text)?;
    file.write_all(memos_text.as_bytes())?;
    file.flush()?;

    Ok(())
}

