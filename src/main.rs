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
        #[clap(short, long)]
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
            write_file(vec!(task)).unwrap();
            Ok(())
        },
        Action::Done { id } => {
            unimplemented!()
        }
        Action::Show {  } => {
            for (i, task) in load_task()? {
                print!("{}| {}", i, task)
            }
            Ok(())
        }
    }
}
