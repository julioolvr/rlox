import React from "react";

class Rlox extends React.Component {
  state = {
    code: `fun getAnswer() {
  return 42;
}

print getAnswer();
    `
  };

  setCode(code) {
    this.setState({ code });
  }

  render() {
    const { interpreter, children } = this.props;

    return (
      <div>
        <textarea
          value={this.state.code}
          onChange={e => this.setCode(e.target.value)}
        />
        {children(interpreter.run(this.state.code))}
        <style jsx>{`
          textarea {
            font-family: monospace;
          }
        `}</style>
      </div>
    );
  }
}

export default Rlox;
