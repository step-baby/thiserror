use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub enum Error {
    #[display(transparent)]
    Other(#[source] anyhow::Error),
}

fn main() {}
