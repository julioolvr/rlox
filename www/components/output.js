const Output = ({ children }) => {
  return (
    <div>
      <h2>Output</h2>
      <hr />
      <pre>{children}</pre>

      <style jsx>{`
        pre {
          font-size: 1.2em;
        }
      `}</style>
    </div>
  );
};

export default Output;
