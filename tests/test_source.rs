use std::error::Error as StdError;
use std::io;
use thiserror::EnumDisplay;

#[derive(EnumDisplay, Debug)]
#[display("implicit source")]
pub struct ImplicitSource {
    source: io::Error,
}

#[derive(EnumDisplay, Debug)]
#[display("explicit source")]
pub struct ExplicitSource {
    source: String,
    #[source]
    io: io::Error,
}

#[derive(EnumDisplay, Debug)]
#[display("boxed source")]
pub struct BoxedSource {
    #[source]
    source: Box<dyn StdError + Send + 'static>,
}

#[test]
fn test_implicit_source() {
    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = ImplicitSource { source: io };
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}

#[test]
fn test_explicit_source() {
    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = ExplicitSource {
        source: String::new(),
        io,
    };
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}

#[test]
fn test_boxed_source() {
    let source = Box::new(io::Error::new(io::ErrorKind::Other, "oh no!"));
    let error = BoxedSource { source };
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}

macro_rules! error_from_macro {
    ($($variants:tt)*) => {
        #[derive(EnumDisplay)]
        #[derive(Debug)]
        pub enum MacroSource {
            $($variants)*
        }
    }
}

// Test that we generate impls with the proper hygiene
#[rustfmt::skip]
error_from_macro! {
    #[display("Something")]
    Variant(#[from] io::Error)
}
