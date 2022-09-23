use thiserror::EnumDisplay;

#[derive(Debug)]
struct NotError;

#[derive(EnumDisplay, Debug)]
#[display("...")]
pub struct ErrorStruct {
    source: NotError,
}

fn main() {}
