#![cfg(unix)]

#[test]
fn ui() {
    trybuild::TestCases::new().compile_fail("./tests/fail/*.rs");
}
