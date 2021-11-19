#[macro_export]
macro_rules! ese {
    ($($e:expr),*) => {
        ::assert_cmd::Command::cargo_bin("ese")
            .expect("Failed to find ese executable")
            $(.arg($e))*
            .unwrap();
    }
}


// fn has_env(key: &str) -> bool {
//     env::var_os(key).is_some()
// }

#[test]
fn add_env() {

}