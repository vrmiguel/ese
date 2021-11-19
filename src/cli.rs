use std::path::{Path, PathBuf};

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// A very lightweight environment variable manager
pub struct Args {
    #[argh(positional)]
    /// the command to be executed.
    pub command: Vec<PathBuf>,

    #[argh(option, short = 'f')]
    /// the dotenv file to be loaded. Defaults to `ese.env` if not set.
    pub file: Option<PathBuf>,

    #[argh(switch, short = 'c')]
    /// if set, the child process receives no env. variables except those contained in the .env file.
    pub clear: bool,

    #[argh(switch, short = 'p')]
    /// if set, the child process receives no env. variables except the parent's PATH and the values contained in the .env file.
    pub path_only: bool,
}


impl Args {
    pub fn input_file(&self) -> &Path {
        self.file.as_deref().unwrap_or_else(|| Path::new("ese.env"))
    }

    pub fn command(&self) -> &[PathBuf] {
        &self.command
    }

    pub fn should_clear(&self) -> bool {
        self.clear
    }

    pub fn child_receives_path_only(&self) -> bool {
        self.path_only
    }
}