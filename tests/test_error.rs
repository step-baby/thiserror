#![allow(dead_code)]

use std::fmt::{self, Display};
use std::io;
use thiserror::EnumDisplay;

macro_rules! unimplemented_display {
    ($ty:ty) => {
        impl Display for $ty {
            fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
                unimplemented!()
            }
        }
    };
}

#[derive(EnumDisplay, Debug)]
struct BracedError {
    msg: String,
    pos: usize,
}

#[derive(EnumDisplay, Debug)]
struct TupleError(String, usize);

#[derive(EnumDisplay, Debug)]
struct UnitError;

#[derive(EnumDisplay, Debug)]
struct WithSource {
    #[source]
    cause: io::Error,
}

#[derive(EnumDisplay, Debug)]
struct WithAnyhow {
    #[source]
    cause: anyhow::Error,
}

#[derive(EnumDisplay, Debug)]
enum EnumError {
    Braced {
        #[source]
        cause: io::Error,
    },
    Tuple(#[source] io::Error),
    Unit,
}

unimplemented_display!(BracedError);
unimplemented_display!(TupleError);
unimplemented_display!(UnitError);
unimplemented_display!(WithSource);
unimplemented_display!(WithAnyhow);
unimplemented_display!(EnumError);
