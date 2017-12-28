import React from "react";
import Head from "next/head";

import Rlox from "../components/rlox";
import Output from "../components/output";
import getInterpreter from "../rlox";

class Index extends React.Component {
  state = {
    interpreter: null,
    loading: true
  };

  componentDidMount() {
    getInterpreter().then(interpreter =>
      this.setState({ loading: false, interpreter })
    );
  }

  render() {
    return (
      <div>
        <Head>
          <title>rlox</title>
        </Head>
        {this.state.loading ? (
          <div>Loading...</div>
        ) : (
          <Rlox interpreter={this.state.interpreter}>
            {output => <Output>{output}</Output>}
          </Rlox>
        )}
      </div>
    );
  }
}

export default Index;
