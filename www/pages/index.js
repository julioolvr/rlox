import React from "react";
import Head from "next/head";

import Rlox from "../components/rlox";
import Output from "../components/output";
import getInterpreter from "../rlox";

const examples = {
  closures: `fun getAnswerer(answer) {
  fun getAnswer() {
    return answer;
  }

  return getAnswer;
}

fun printer(fn) {
  print fn();
}

printer(getAnswerer(42));`,
  classes: `class Parent {
  sum(a, b) {
    return a + b;
  }
}

class Child < Parent {
  init(n) {
    this.n = n;
  }

  sumN(a) {
    return super.sum(a, this.n);
  }
}

var child = Child(3);
print child.sumN(2);`,
  recursion: `fun fibo(n) {
  if (n == 0) {
    return 1;
  }

  if (n == 1) {
    return 1;
  }

  return fibo(n - 1) + fibo(n - 2);
}

// Don't try this one with a number too big unless you
// want to freeze your browser :)
print fibo(5);`
};

class Index extends React.Component {
  state = {
    interpreter: null,
    loading: true,
    code: `fun getAnswer() {
  return 42;
}

print getAnswer();`
  };

  componentDidMount() {
    getInterpreter().then(interpreter =>
      this.setState({ loading: false, interpreter })
    );
  }

  setCode(code) {
    this.setState({ code });
  }

  render() {
    return (
      <div className="container">
        <Head>
          <title>rlox</title>
          <meta name="viewport" content="width=device-width, initial-scale=1" />
        </Head>

        <div className="content">
          <h1>rlox.wasm</h1>

          <div>
            <p>
              <a href="http://www.craftinginterpreters.com/">Lox interpreter</a>
              <br />
              <a href="http://github.com/julioolvr/rlox">Written in Rust</a>
              <br />
              And compiled to WebAssembly
              <br />
              by <a href="http://joliv.me/">@julioolvr</a>
            </p>
          </div>

          {this.state.loading ? (
            <div>Loading interpreter...</div>
          ) : (
            <Rlox
              interpreter={this.state.interpreter}
              code={this.state.code}
              onCodeChange={newCode => this.setCode(newCode)}
            >
              {output => (
                <React.Fragment>
                  <h2>Examples</h2>
                  <div>
                    <a onClick={() => this.setCode(examples.closures)}>
                      Closures
                    </a>
                    {" - "}
                    <a onClick={() => this.setCode(examples.classes)}>
                      Classes
                    </a>
                    {" - "}
                    <a onClick={() => this.setCode(examples.recursion)}>
                      Recursion
                    </a>
                  </div>
                  <Output>{output}</Output>
                </React.Fragment>
              )}
            </Rlox>
          )}
        </div>

        <style jsx>{`
          .container {
            display: flex;
            justify-content: center;
          }

          .content {
            width: 40em;
            padding-bottom: 2em;
          }

          a {
            cursor: pointer;
          }
        `}</style>

        <style jsx global>{`
          @import url("https://fonts.googleapis.com/css?family=Roboto+Mono:300,700|Roboto:300,700");
          * {
            box-sizing: border-box;
          }
          ::selection {
            background-color: #16146c;
          }
          ::-moz-selection {
            background-color: #16146c;
          }
          html,
          body {
            margin: 0;
            padding: 0;
          }
          body {
            background-color: #0e0e0e;
            color: #e3e3e3;
            font-family: "Roboto Mono", monospace;
          }
          a {
            color: #af3eff;
            text-decoration: none;
          }
          a:hover {
            text-decoration: underline;
          }
        `}</style>
      </div>
    );
  }
}

export default Index;
