mod cli;
mod process;

pub use cli::{Base64Format, Base64Options, Options, SubCommand};
pub use process::{generate_password, process_csv, process_decode, process_encode};
