use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub target_function: String,
}

#[allow(dead_code)]
pub fn get_args() -> Cli {
    Cli::parse()
}
