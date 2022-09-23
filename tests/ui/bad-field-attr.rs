use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display(transparent)]
pub struct Error(#[display(transparent)] std::io::Error);

fn main() {}
