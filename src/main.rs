use crate::uci::uci::UCICommand;

mod uci;

fn main() {
    // Launch UCI protocol
    UCICommand::run();
}
