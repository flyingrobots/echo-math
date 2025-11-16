#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();

    // Test all fail-cases in this folder
    t.compile_fail("tests/compile/compile_fail/*.rs");
}
