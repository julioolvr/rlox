extern crate rlox;

mod utils;

#[test]
fn closure_environment_mutation() {
    let output = utils::execute(
        r#"
        var a = "global";
        {
            fun showA() {
                print a;
            }

            showA();
            var a = "block";
            showA();
        }
    "#,
    );

    assert_eq!(output[0], "global");
    assert_eq!(
        output[1],
        "global",
        "Assignments to enclosed variable shouldn't change function closure environment"
    );
}
