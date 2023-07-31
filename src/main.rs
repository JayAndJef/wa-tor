pub mod sim;
pub mod states;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    prey_init: Option<u32>,

    pred_init: Option<u32>,

    prey_rep: Option<u32>,

    pred_rep: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
}
