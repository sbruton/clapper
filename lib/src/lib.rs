pub use clapper_macro::main;
pub use ctrlc;

pub trait ClapperError: std::error::Error {
    fn exit_code(&self) -> i32;
}

pub type ClapperResult<E> = std::result::Result<(), E>;

pub mod prelude {
    pub use clap::{self, ArgAction, ArgGroup, Parser as ArgParser};
    pub use ctrlc;

    pub use super::ClapperError;
}
