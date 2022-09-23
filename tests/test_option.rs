#![cfg_attr(
    thiserror_nightly_testing,
    feature(error_generic_member_access, provide_any)
)]

#[cfg(thiserror_nightly_testing)]
pub mod structs {
    use std::backtrace::Backtrace;
    use thiserror::EnumDisplay;

    #[derive(EnumDisplay, Debug)]
    #[display("...")]
    pub struct OptSourceNoBacktrace {
        #[source]
        source: Option<anyhow::Error>,
    }

    #[derive(EnumDisplay, Debug)]
    #[display("...")]
    pub struct OptSourceAlwaysBacktrace {
        #[source]
        source: Option<anyhow::Error>,
        backtrace: Backtrace,
    }

    #[derive(EnumDisplay, Debug)]
    #[display("...")]
    pub struct NoSourceOptBacktrace {
        #[backtrace]
        backtrace: Option<Backtrace>,
    }

    #[derive(EnumDisplay, Debug)]
    #[display("...")]
    pub struct AlwaysSourceOptBacktrace {
        source: anyhow::Error,
        #[backtrace]
        backtrace: Option<Backtrace>,
    }

    #[derive(EnumDisplay, Debug)]
    #[display("...")]
    pub struct OptSourceOptBacktrace {
        #[source]
        source: Option<anyhow::Error>,
        #[backtrace]
        backtrace: Option<Backtrace>,
    }
}

#[cfg(thiserror_nightly_testing)]
pub mod enums {
    use std::backtrace::Backtrace;
    use thiserror::EnumDisplay;

    #[derive(EnumDisplay, Debug)]
    pub enum OptSourceNoBacktrace {
        #[display("...")]
        Test {
            #[source]
            source: Option<anyhow::Error>,
        },
    }

    #[derive(EnumDisplay, Debug)]
    pub enum OptSourceAlwaysBacktrace {
        #[display("...")]
        Test {
            #[source]
            source: Option<anyhow::Error>,
            backtrace: Backtrace,
        },
    }

    #[derive(EnumDisplay, Debug)]
    pub enum NoSourceOptBacktrace {
        #[display("...")]
        Test {
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }

    #[derive(EnumDisplay, Debug)]
    pub enum AlwaysSourceOptBacktrace {
        #[display("...")]
        Test {
            source: anyhow::Error,
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }

    #[derive(EnumDisplay, Debug)]
    pub enum OptSourceOptBacktrace {
        #[display("...")]
        Test {
            #[source]
            source: Option<anyhow::Error>,
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }
}

#[test]
#[cfg_attr(not(thiserror_nightly_testing), ignore)]
fn test_option() {}
