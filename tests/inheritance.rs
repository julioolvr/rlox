extern crate rlox;
mod utils;
use utils::execute;

#[test]
fn expects_superclass_name_after_less_token() {
    let output = execute("class Child < {}");
    assert!(output[0].contains("Expected superclass name"));
}

#[test]
fn can_call_superclass_method() {
    let output = execute(
        r#"
        class Parent {
            getAnswer() {
                return 42;
            }
        }

        class Child < Parent {}

        print Child().getAnswer();
    "#,
    );

    assert_eq!(output[0], "42");
}
