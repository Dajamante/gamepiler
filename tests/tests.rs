use gamepiler::check_and_build;

fn get_correct_errors(path: &str) -> Vec<String> {
    let p = String::from("tests/deffective/") + path + "/Cargo.toml";
    let errors = check_and_build::parsing_errors(&p);
    errors
}

#[test]
fn test_e0004() {
    assert!(get_correct_errors("E0004").contains(&"E0004".to_owned()));
}

#[test]
fn test_e0425() {
    assert!(get_correct_errors("E0425").contains(&"E0425".to_owned()));
}
