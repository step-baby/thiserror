use thiserror::EnumDisplay;

#[derive(Debug)]
struct NoDisplay;

#[derive(EnumDisplay, Debug)]
#[display("thread: {thread}")]
pub struct Error {
    thread: NoDisplay,
}

fn main() {}
