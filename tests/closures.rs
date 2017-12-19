extern crate rlox;
mod utils;

#[test]
#[ignore]
fn simple_closure() {
    let output = utils::execute(
        r#"
        fun giveMeAClosure() {
            var answer = 42;

            fun closure() {
                return answer;
            }

            return closure;
        }

        var gotClosure = giveMeAClosure();
        print gotClosure();
    "#,
    );

    assert_eq!(output[0], "42");
}
