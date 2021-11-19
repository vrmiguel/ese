use std::io::Write;

use predicates::prelude::*;

#[macro_export]
macro_rules! ese {
    ($($e:expr),*) => {
        ::assert_cmd::Command::cargo_bin("ese")
            .expect("Failed to find ese executable")
            $(.arg($e))*
    }
}

fn has_key(key: &str) -> bool {
    std::env::var_os(key).is_some()
}

#[test]
fn echo() {
    if has_key("FOOBAR123") {
        panic!("FOOBAR123 should be reserved for testing")
    }

    let mut echo_script = tempfile::NamedTempFile::new().unwrap();

    writeln!(echo_script, "#!/bin/bash").unwrap();
    writeln!(echo_script, "echo $FOOBAR123").unwrap();

    echo_script.flush().unwrap();

    let mut dotenv = tempfile::NamedTempFile::new().unwrap();
    writeln!(dotenv, "FOOBAR123=XYZ").unwrap();

    dotenv.flush().unwrap();

    ese!("bash", echo_script.path(), "-f", dotenv.path())
        .assert()
        .stdout(predicate::eq(b"XYZ\n" as &[u8]));
}
