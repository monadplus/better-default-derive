#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/good/*.rs");
    t.compile_fail("tests/bad/*.rs");
}
