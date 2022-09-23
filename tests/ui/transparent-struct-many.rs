use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display(transparent)]
pub struct Error {
    inner: anyhow::Error,
    what: String,
}

fn main() {}
