import React from "react";

class Rlox extends React.Component {
  render() {
    const { code, onCodeChange, interpreter, children } = this.props;

    return (
      <div>
        <textarea
          autoFocus
          value={code}
          onChange={e => onCodeChange(e.target.value)}
        />
        {children(interpreter.run(code))}
        <style jsx>{`
          textarea {
            font-family: "Roboto mono", serif;
            font-size: 16px;
            width: 100%;
            height: 15em;
            background-color: transparent;
            color: #a7fe92;
          }
        `}</style>
      </div>
    );
  }
}

export default Rlox;
