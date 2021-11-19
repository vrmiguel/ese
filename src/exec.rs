use std::env as std_env;
use std::os::unix::prelude::ExitStatusExt;
use std::process::ExitStatus;
use std::{fmt::Display, process::Command};

use anyhow::{bail, Context, Result};

use crate::{cli::Args, env};

pub fn exec_with_env(data: Args) -> Result<TerminationStatus> {
    // The user-supplied command passed in
    let commands = data.command();

    // The .env file we'll read from
    let input_path = data.input_file();

    // If set, the child process will receive no env. variables from the parent process
    let should_clear = data.should_clear();

    // If set, the child process receives only the PATH of the parent process
    let path_only = data.child_receives_path_only();

    // The name of the program to be executed
    let program = commands.get(0).with_context(|| "No command to run")?;

    // Early assign in order to make the Command not be dropped too early
    let mut command = Command::new(program);
    let mut command = command.args(commands.iter().skip(1));

    // Read the values from the .env file
    let key_values = env::read_from_dotenv(input_path)?;

    command = match (should_clear, path_only) {
        (true, true) => {
            // These flags contradict themselves
            //
            // The child must either get no envs from the parent or get only the PATH
            bail!("--clear contradicts with --path-only");
        }
        (false, false) => command, // Child will inherit all env vars from the parent
        (true, false) => command.env_clear(),
        (false, true) => {
            command.env_clear();
            let path = std_env::var_os("PATH").with_context(|| "PATH not found!")?;
            command.env("PATH", path)
        }
    };

    // Set the env. variables read from the .env file to the child process
    let command = command.envs(key_values);

    let mut child = command.spawn()?;

    let exit_status = child.wait()?;

    Ok(exit_status.into())
}

/// Represents the termination status of a command
pub enum TerminationStatus {
    /// If the given command was interrupted through a signal, this variant holds the value of that signal
    Signaled(i32),
    /// If the command terminated normally (i.e. not signal interrupted), this variant holds the exit code of the command
    TerminatedNormally(i32),
}

impl TerminationStatus {
    /// Returns true if the command terminated normally and with exit code 0
    pub fn is_ok(&self) -> bool {
        matches!(self, TerminationStatus::TerminatedNormally(0))
    }

    /// Returns either the code of this command (if terminated normally) or the value of the signal that killed it (if signal interrupted)
    pub fn code_or_signal(&self) -> i32 {
        match self {
            TerminationStatus::Signaled(signal) => *signal,
            TerminationStatus::TerminatedNormally(code) => *code,
        }
    }
}

impl Display for TerminationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminationStatus::Signaled(signal) => {
                write!(f, "The command was terminated from signal {}", signal)
            }
            TerminationStatus::TerminatedNormally(exit_code) => write!(
                f,
                "The command terminated normally with exit code {}",
                exit_code
            ),
        }
    }
}

impl From<ExitStatus> for TerminationStatus {
    fn from(exit_status: ExitStatus) -> Self {
        match exit_status.signal() {
            Some(signal) => TerminationStatus::Signaled(signal),
            None => {
                // Safety: the docs state that, on Unix, ExitStatus::code will only fail if
                //         the process was killed from a signal. We've just checked that this is not the case.
                let exit_code = exit_status.code().unwrap();
                TerminationStatus::TerminatedNormally(exit_code)
            }
        }
    }
}
