//! Exports the current environment variables to a dotenv file

use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Context, Result};
use fs_err as fs;

/// Reads env. variables read from a dotenv file given by `path` into a Vec of keys and values
pub fn read_from_dotenv(path: &Path) -> Result<Vec<(String, String)>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut key_values = vec![];

    for line in reader.lines() {
        let line = line?;
        let (key, value) = line
            .split_once('=')
            .with_context(|| format!("{} is not in the expected KEY=VALUE format", line))?;

        // Make sure both key and value do not have a NUL byte
        key.err_if_contains_nil()?;
        value.err_if_contains_nil()?;

        let key_value = (key.to_string(), value.to_string());

        key_values.push(key_value);
    }

    Ok(key_values)
}

trait KeyValueValidator {
    fn err_if_contains_nil(&self) -> Result<()>;
}

impl KeyValueValidator for str {
    fn err_if_contains_nil(&self) -> Result<()> {
        if self.contains('\0') {
            bail!(format!("{} contains a NUL byte", self))
        } else {
            Ok(())
        }
    }
}
