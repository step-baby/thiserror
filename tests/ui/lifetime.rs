use std::fmt::Debug;
use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display("error")]
struct Error<'a>(#[from] Inner<'a>);

#[derive(EnumDisplay, Debug)]
#[display("{0}")]
struct Inner<'a>(&'a str);

#[derive(EnumDisplay, Debug)]
enum Enum<'a> {
    #[display("error")]
    Foo(#[from] Generic<&'a str>),
}

#[derive(EnumDisplay, Debug)]
#[display("{0:?}")]
struct Generic<T: Debug>(T);

fn main() -> Result<(), Error<'static>> {
    Err(Error(Inner("some text")))
}
