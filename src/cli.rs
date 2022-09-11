use clap::Parser;

#[derive(Parser)]
#[clap(about = "A simple tool to find EVM function signature collisions")]
pub struct Cli {
    #[clap(short, long, default_value = "find")]
    mode: String,
    //todo
}

pub fn get_args() -> Cli {
    Cli::parse()
}
