use rum::execution::{UniversalMachine, execute};
use rum::rumload;
use std::env;

fn main() {
    // Fetch the input file from command line arguments
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut um = UniversalMachine::new(&instructions);
    execute(&mut um);
   
}

