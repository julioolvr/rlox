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
    let output = utils::execute(
        r#"
        class Something {}
        print Something;
    "#,
    );

    assert_eq!(output[0], "class <Something>");
}

#[test]
fn printing_class_instance_shows_class_name() {
    let output = utils::execute(
        r#"
        class Something {}
        var instance = Something();
        print instance;
    "#,
    );

    assert_eq!(output[0], "instance of <Something>");
}

#[test]
fn read_property_from_instance_is_a_runtime_error() {
    let output = utils::execute(
        r#"
        class Something {}
        var instance = Something();
        print instance.someProperty;
    "#,
    );

    assert!(output[0].ends_with("Undefined property `someProperty`."));
}

#[test]
fn set_property_in_instance() {
    let output = utils::execute(
        r#"
        class Something {}
        var instance = Something();
        instance.someProperty = 42;
        print instance.someProperty;
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
fn call_method_on_instance() {
    let output = utils::execute(
        r#"
        class DeepThought {
            getAnswer() {
                return 42;
            }
        }

        var instance = DeepThought();
        print instance.getAnswer();
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
fn reach_this_from_instance_method() {
    let output = utils::execute(
        r#"
        class DeepThought {
            getAnswer() {
                return this.answer;
            }
        }

        var instance = DeepThought();
        instance.answer = 42;
        print instance.getAnswer();
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
fn detached_this() {
    let output = utils::execute(
        r#"
        class DeepThought {
            getAnswer() {
                return this.answer;
            }
        }

        var instance = DeepThought();
        instance.answer = 42;
        var method = instance.getAnswer;
        print method();
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
fn callback_with_bound_this() {
    let output = utils::execute(
        r#"
        class DeepThought {
            getCallback() {
                fun callback() {
                    return this.answer;
                }

                return callback;
            }
        }

        var instance = DeepThought();
        instance.answer = 42;
        print instance.getCallback()();
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
#[should_panic(expected = "UnexpectedTokenError: Cannot use `this` outside of a method.")]
fn using_this_in_root_fails() {
    utils::execute(
        r#"
        print this;
    "#,
    );
}

#[test]
#[should_panic(expected = "UnexpectedTokenError: Cannot use `this` outside of a method.")]
fn using_this_in_non_method_fails() {
    utils::execute(
        r#"
        fun thisShouldFail() {
            print this;
        }
    "#,
    );
}

#[test]
fn instance_initializer() {
    let output = utils::execute(
        r#"
        class DeepThought {
            init() {
                this.answer = 42;
            }
        }

        print DeepThought().answer;
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
fn initializer_with_parameters() {
    let output = utils::execute(
        r#"
        class DeepThought {
            init(answer) {
                this.answer = answer;
            }
        }

        print DeepThought(42).answer;
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
fn calling_init_returns_the_same_instance() {
    let output = utils::execute(
        r#"
        class DeepThought {
            init() {
                this.answer = 42;
            }
        }

        var instance = DeepThought();
        instance.answer = 43;
        print instance.init().answer;
    "#,
    );

    assert_eq!(output[0], "42");
}

#[test]
#[should_panic(expected = "UnexpectedTokenError: Cannot use `return` on an initializer.")]
fn return_from_init_throws_an_error() {
    utils::execute(
        r#"
        class DeepThought {
            init() {
                return 42;
            }
        }
    "#,
    );
}

#[test]
fn compared_by_identity() {
    let output = utils::execute(
        r#"
        class A {}

        var B = A;
        print A == B;
    "#,
    );

    assert_eq!(output[0], "true");
}

#[test]
fn compare_instances_by_identity() {
    let output = utils::execute(
        r#"
        class A {}

        var a = A();
        var b = a;
        print a == b;
    "#,
    );

    assert_eq!(output[0], "true");
}
