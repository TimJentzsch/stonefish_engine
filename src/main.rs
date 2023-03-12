#![feature(str_split_whitespace_remainder)]
use stonefish::Stonefish;
use uci::UciRunner;

mod stonefish;
mod uci;

fn main() {
    // Launch UCI protocol
    UciRunner::run::<Stonefish>();
}
