mod startup;
mod model;
mod convert;

use clap::Parser;
use crate::startup::{Args, run};

fn main() {
    let args = Args::parse();
    if run(args).is_ok() {};
}
