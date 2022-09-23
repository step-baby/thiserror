use thiserror::EnumDisplay;

#[derive(EnumDisplay)]
pub union U {
    msg: &'static str,
    num: usize,
}

fn main() {}
