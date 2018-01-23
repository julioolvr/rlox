extern crate rlox;
mod utils;
use utils::execute;

#[test]
fn function_parameters() {
    let output = execute(
        r#"
      fun sum(a, b) {
          return a + b;
      }

      print sum(1, 2);
    "#,
    );

    assert_eq!(output[0], "3");
}

#[test]
#[should_panic(expected = "UnexpectedTokenError: Cannot use `return` at the top level.")]
fn return_on_top_level() {
    execute("return 42;");
}

#[test]
fn compared_by_identity() {
    let output = execute(
        r#"
      fun sum(a, b) {
          return a + b;
      }

      var b = sum;
      print b == sum;
    "#,
    );

    assert_eq!(output[0], "true");
}
