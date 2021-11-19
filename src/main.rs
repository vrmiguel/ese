mod cli;
mod env;
mod exec;

fn try_main() -> anyhow::Result<i32> {
    let args: cli::Args = argh::from_env();

    let mut exit_code = 0;

    let status = exec::exec_with_env(args)?;

    use exec::TerminationStatus::{Signaled, TerminatedNormally};

    if !status.is_ok() {
        match status {
            Signaled(signal) => eprintln!("Child process was signaled with signal {}", signal),
            TerminatedNormally(exit_code) => {
                eprintln!("Child process exited with code {}", exit_code)
            }
        }

        exit_code = status.code_or_signal();
    }

    Ok(exit_code)
}

fn main() {
    match try_main() {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
