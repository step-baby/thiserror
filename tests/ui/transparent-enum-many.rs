use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub enum Error {
    #[display(transparent)]
    Other(anyhow::Error, String),
}

fn main() {}
