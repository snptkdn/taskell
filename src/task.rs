use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f32::consts::E;
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

pub fn load_task() -> Vec<(usize, Task)> {
    unimplemented!()
}

pub fn write_file(tasks: Vec<Task>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("task.json")?;

    let mut tasks_map = HashMap::new();
    for (i, task) in tasks.iter().enumerate() {
        tasks_map.insert(i+1, task);
    }

    let memos_text = serde_json::to_string(&tasks_map)?;
    write!(&file, "{}", memos_text)?;

    Ok(())
}

