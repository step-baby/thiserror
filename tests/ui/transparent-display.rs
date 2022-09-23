use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display(transparent)]
#[display("...")]
pub struct Error(anyhow::Error);

fn main() {}
