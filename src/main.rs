use stonefish::Stonefish;
use uci::UciRunner;

mod stonefish;
mod uci;

fn main() {
    // Launch UCI protocol
    UciRunner::run::<Stonefish>();
}
