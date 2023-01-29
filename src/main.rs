use clap::{Parser, Subcommand};
use anyhow::Result;

mod task;
use task::*;

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
