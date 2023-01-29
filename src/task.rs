use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use chrono::{Local, DateTime};

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
            write!(f, "{:<50}|{:>5}", self.title, point)
        } else {
            write!(f, "{:<50}|{:>5}", self.title, "-")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoneTask {
    point: Option<usize>,
    date: DateTime<Local>,

}

impl DoneTask {
    pub fn new(point: Option<usize>, date: DateTime<Local>) -> Self {
        Self {
            point,
            date
        }
    }
}

pub fn delete_task(mut tasks: HashMap<usize, Task>, id: usize) -> Result<HashMap<usize, Task>> {
    println!("before delete {:?}", &tasks);
    let task = tasks.remove(&id);
    if let Some(task) = task {
        archive_task(task)?;
        Ok(tasks)
    } else {
        Ok(tasks)
    }
}

fn archive_task(task: Task) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("archives.json")?;

    println!("archive");
    let local_datetime: DateTime<Local> = Local::now();
    let done_task = DoneTask::new(task.point, local_datetime);
    let done_task = serde_json::to_string(&done_task)?;
    println!("{:?}", done_task);
    file.write_all(done_task.as_bytes())?;
    Ok(())
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

