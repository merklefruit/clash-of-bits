use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short = 'm', long)]
    pub mode: String,
}

#[allow(dead_code)]
pub fn get_args() -> Cli {
    Cli::parse()
}
