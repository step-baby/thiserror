use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display(transparent)]
#[display(transparent)]
pub struct Error(anyhow::Error);

fn main() {}
