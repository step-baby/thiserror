use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub struct ErrorStruct {
    #[source]
    a: std::io::Error,
    #[source]
    b: anyhow::Error,
}

fn main() {}
