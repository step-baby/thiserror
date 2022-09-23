use thiserror::EnumDisplay;

macro_rules! error_type {
    ($name:ident, $what:expr) => {
        // Use #[display("invalid {}", $what)] instead.

        #[derive(EnumDisplay, Debug)]
        #[display(concat!("invalid ", $what))]
        pub struct $name;
    };
}

error_type!(Error, "foo");

fn main() {}
