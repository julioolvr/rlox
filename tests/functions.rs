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
