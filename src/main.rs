use stonefish::stonefish::Stonefish;

use crate::uci::uci::UciCommand;

mod stonefish;
mod uci;

fn main() {
    let engine: &'static Stonefish = &stonefish::stonefish::Stonefish {};
    // Launch UCI protocol
    UciCommand::run(engine);
}
