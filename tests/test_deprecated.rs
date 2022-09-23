#![deny(deprecated, clippy::all, clippy::pedantic)]

use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
pub enum Error {
    #[deprecated]
    #[display("...")]
    Deprecated,
}
