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

#[test]
fn overriding_methods() {
    let output = execute(
        r#"
        class Parent {
            getValue() {
                return 1;
            }
        }

        class Child {
            getValue() {
                return 2;
            }
        }

        print Child().getValue();
    "#,
    );

    assert_eq!(output[0], "2");
}

#[test]
fn using_super() {
    let output = execute(
        r#"
        class Parent {
            getValue() {
                return 1;
            }
        }

        class Child < Parent {
            getValue() {
                return super.getValue() + 2;
            }
        }

        print Child().getValue();
    "#,
    );

    assert_eq!(output[0], "3");
}

#[test]
fn referencing_super_without_access_fails() {
    let output = execute(
        r#"
        class Something {
            getValue() {
                return super;
            }
        }
    "#,
    );

    assert!(output[0].contains("Expect '.' after 'super'."));
}

#[test]
#[should_panic(expected = "UnexpectedTokenError: Cannot use `super` outside of a method.")]
fn using_super_outside_of_method_fails() {
    execute("super.doSomething();");
}

#[test]
#[should_panic(expected = "UnexpectedTokenError: Cannot use `super` without a superclass.")]
fn using_super_without_superclass_fails() {
    execute(
        r#"
        class Something {
            getValue() {
                return super.getValue();
            }
        }
    "#,
    );
}
