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

#[test]
fn printing_class_instance_shows_class_name() {
    let output = utils::execute(r#"
        class Something {}
        var instance = Something();
        print instance;
    "#);

    assert_eq!(output[0], "instance of <Something>");
}

#[test]
fn read_property_from_instance_is_a_runtime_error() {
    let output = utils::execute(r#"
        class Something {}
        var instance = Something();
        print instance.someProperty;
    "#);

    assert!(output[0].ends_with("Undefined property `someProperty`."));
}

#[test]
fn set_property_in_instance() {
    let output = utils::execute(r#"
        class Something {}
        var instance = Something();
        instance.someProperty = 42;
        print instance.someProperty;
    "#);

    assert_eq!(output[0], "42");
}