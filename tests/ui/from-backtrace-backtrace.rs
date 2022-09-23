// https://github.com/dtolnay/thiserror/issues/163

use std::backtrace::Backtrace;
use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display("...")]
pub struct Error(#[from] #[backtrace] std::io::Error, Backtrace);

fn main() {}
