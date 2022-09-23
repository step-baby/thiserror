use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub enum Error {
    #[display("...")]
    A(usize),
    B(usize),
}

fn main() {}
