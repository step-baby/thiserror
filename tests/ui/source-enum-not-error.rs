use thiserror::EnumDisplay;

#[derive(Debug)]
pub struct NotError;

#[derive(EnumDisplay, Debug)]
#[display("...")]
pub enum ErrorEnum {
    Broken {
        source: NotError,
    },
}

fn main() {}
