use clap::{Parser, Subcommand};

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
    }
}

fn main() {
    let cli = AppArg::parse();
    match cli.action {
        Action::Add { title, point } => {
            let task = Task::new(title, point);
        },
        Action::Done { id } => {
        }
    }
}
