use std::fmt::Display;
use thiserror::EnumDisplay;

fn assert<T: Display>(expected: &str, value: T) {
    assert_eq!(expected, value.to_string());
}

#[test]
fn test_braced() {
    #[derive(EnumDisplay, Debug)]
    #[display("braced error: {msg}")]
    struct Error {
        msg: String,
    }

    let msg = "T".to_owned();
    assert("braced error: T", Error { msg });
}

#[test]
fn test_braced_unused() {
    #[derive(EnumDisplay, Debug)]
    #[display("braced error")]
    struct Error {
        extra: usize,
    }

    assert("braced error", Error { extra: 0 });
}

#[test]
fn test_tuple() {
    #[derive(EnumDisplay, Debug)]
    #[display("tuple error: {0}")]
    struct Error(usize);

    assert("tuple error: 0", Error(0));
}

#[test]
fn test_unit() {
    #[derive(EnumDisplay, Debug)]
    #[display("unit error")]
    struct Error;

    assert("unit error", Error);
}

#[test]
fn test_enum() {
    #[derive(EnumDisplay, Debug)]
    enum Error {
        #[display("braced error: {id}")]
        Braced { id: usize },
        #[display("tuple error: {0}")]
        Tuple(usize),
        #[display("unit error")]
        Unit,
    }

    assert("braced error: 0", Error::Braced { id: 0 });
    assert("tuple error: 0", Error::Tuple(0));
    assert("unit error", Error::Unit);
}

#[test]
fn test_constants() {
    #[derive(EnumDisplay, Debug)]
    #[display("{MSG}: {id:?} (code {CODE:?})")]
    struct Error {
        id: &'static str,
    }

    const MSG: &str = "failed to do";
    const CODE: usize = 9;

    assert("failed to do: \"\" (code 9)", Error { id: "" });
}

#[test]
fn test_inherit() {
    #[derive(EnumDisplay, Debug)]
    #[display("{0}")]
    enum Error {
        Some(&'static str),
        #[display("other error")]
        Other(&'static str),
    }

    assert("some error", Error::Some("some error"));
    assert("other error", Error::Other("..."));
}

#[test]
fn test_brace_escape() {
    #[derive(EnumDisplay, Debug)]
    #[display("fn main() {{}}")]
    struct Error;

    assert("fn main() {}", Error);
}

#[test]
fn test_expr() {
    #[derive(EnumDisplay, Debug)]
    #[display("1 + 1 = {}", 1 + 1)]
    struct Error;
    assert("1 + 1 = 2", Error);
}

#[test]
fn test_nested() {
    #[derive(EnumDisplay, Debug)]
    #[display("!bool = {}", not(.0))]
    struct Error(bool);

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn not(bool: &bool) -> bool {
        !*bool
    }

    assert("!bool = false", Error(true));
}

#[test]
fn test_match() {
    #[derive(EnumDisplay, Debug)]
    #[display("{}: {0}", match .1 {
        Some(n) => format!("error occurred with {}", n),
        None => "there was an empty error".to_owned(),
    })]
    struct Error(String, Option<usize>);

    assert(
        "error occurred with 1: ...",
        Error("...".to_owned(), Some(1)),
    );
    assert(
        "there was an empty error: ...",
        Error("...".to_owned(), None),
    );
}

#[test]
fn test_void() {
    #[allow(clippy::empty_enum)]
    #[derive(EnumDisplay, Debug)]
    #[display("...")]
    pub enum Error {}

    let _: Error;
}

#[test]
fn test_mixed() {
    #[derive(EnumDisplay, Debug)]
    #[display("a={a} :: b={} :: c={c} :: d={d}", 1, c = 2, d = 3)]
    struct Error {
        a: usize,
        d: usize,
    }

    assert("a=0 :: b=1 :: c=2 :: d=3", Error { a: 0, d: 0 });
}

#[test]
fn test_ints() {
    #[derive(EnumDisplay, Debug)]
    enum Error {
        #[display("error {0}")]
        Tuple(usize, usize),
        #[display("error {0}", '?')]
        Struct { v: usize },
    }

    assert("error 9", Error::Tuple(9, 0));
    assert("error ?", Error::Struct { v: 0 });
}

#[test]
fn test_trailing_comma() {
    #[derive(EnumDisplay, Debug)]
    #[display(
        "error {0}",
    )]
    #[rustfmt::skip]
    struct Error(char);

    assert("error ?", Error('?'));
}

#[test]
fn test_field() {
    #[derive(Debug)]
    struct Inner {
        data: usize,
    }

    #[derive(EnumDisplay, Debug)]
    #[display("{}", .0.data)]
    struct Error(Inner);

    assert("0", Error(Inner { data: 0 }));
}

#[test]
fn test_macro_rules() {
    // Regression test for https://github.com/dtolnay/thiserror/issues/86

    macro_rules! decl_error {
        ($variant:ident($value:ident)) => {
            #[derive(Debug, EnumDisplay)]
            pub enum Error0 {
                #[display("{0:?}")]
                $variant($value),
            }

            #[derive(Debug, EnumDisplay)]
            #[display("{0:?}")]
            pub enum Error1 {
                $variant($value),
            }
        };
    }

    decl_error!(Repro(u8));

    assert("0", Error0::Repro(0));
    assert("0", Error1::Repro(0));
}

#[test]
fn test_raw() {
    #[derive(EnumDisplay, Debug)]
    #[display("braced raw error: {r#fn}")]
    struct Error {
        r#fn: &'static str,
    }

    assert("braced raw error: T", Error { r#fn: "T" });
}

#[test]
fn test_raw_enum() {
    #[derive(EnumDisplay, Debug)]
    enum Error {
        #[display("braced raw error: {r#fn}")]
        Braced { r#fn: &'static str },
    }

    assert("braced raw error: T", Error::Braced { r#fn: "T" });
}

#[test]
fn test_raw_conflict() {
    #[derive(EnumDisplay, Debug)]
    enum Error {
        #[display("braced raw error: {r#func}, {func}", func = "U")]
        Braced { r#func: &'static str },
    }

    assert("braced raw error: T, U", Error::Braced { r#func: "T" });
}

#[test]
fn test_keyword() {
    #[derive(EnumDisplay, Debug)]
    #[display("error: {type}", type = 1)]
    struct Error;

    assert("error: 1", Error);
}
