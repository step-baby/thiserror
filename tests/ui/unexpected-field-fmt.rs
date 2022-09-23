use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub enum Error {
    What {
        #[display("...")]
        io: std::io::Error,
    },
}

fn main() {}
