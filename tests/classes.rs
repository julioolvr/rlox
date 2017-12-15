extern crate rlox;
mod utils;

#[test]
fn class_declaration_doesnt_fail() {
    let output = utils::execute("class Something {}");

    assert_eq!(output.len(), 0);
}

#[test]
fn class_declaration_missing_open_brace() {
    let output = utils::execute("class Something");
    assert!(output[0].ends_with("Expected `{` before class body."));
}

#[test]
fn class_declaration_missing_closing_brace() {
    let output = utils::execute("class Something {");
    assert!(output[0].ends_with("Expected `}` after class body."));
}

#[test]
fn printing_class_shows_class_name() {
    let output = utils::execute(r#"
        class Something {}
        print Something;
    "#);

    assert_eq!(output[0], "class <Something>");
}