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

    pub fn display(self) -> String {
        if let Some(point) = self.point {
            format!("{}|{}", self.title, point)
        } else {
            format!("{}", self.title)
        }
    }
}

pub fn load_task() -> Vec<(usize, Task)> {
    unimplemented!()
}

