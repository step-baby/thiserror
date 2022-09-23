use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display(transparent)]
pub struct Error(#[source] anyhow::Error);

fn main() {}
