pub mod model;
pub mod controller;
pub mod cli;

fn main() {
    // TODO let input_path = std::env::args().nth(1).unwrap();
    cli::main_menu();
}