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

