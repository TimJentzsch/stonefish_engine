#![feature(str_split_whitespace_as_str)]
use stonefish::stonefish::Stonefish;
use uci::UciRunner;

mod stonefish;
mod uci;

fn main() {
    // Launch UCI protocol
    UciRunner::run::<Stonefish>();
}
