use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "My Application",
    author = "Author's name",
    version = "v1.0.0",
    about = "Application short description."
)]
struct AppArg {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(short = 'c', long = "count")]
    count: i32,

    message: String,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    for _ in 0..arg.count {
        println!(
            "{}: {}",
            arg.name.clone().unwrap_or(String::from("Alice")),
            arg.message
        );
    }
}
