use stonefish::stonefish::Stonefish;
use uci::uci::{UciRunner};

mod stonefish;
mod uci;

fn main() {
    // Launch UCI protocol
    UciRunner::run::<Stonefish>();
}
