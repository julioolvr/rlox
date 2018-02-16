import wasmLibPath from "../src/lib.rs";

class RloxInterpreter {
  constructor(wasmModuleExports) {
    this.wasm = {
      alloc: wasmModuleExports.alloc,
      run: wasmModuleExports.run_from_wasm,
      memory: wasmModuleExports.memory
    };
  }

  run(code) {
    const input = this.storeStringInBuffer(code);
    return this.readStringFromBuffer(this.wasm.run(input));
  }

  storeStringInBuffer(code) {
    const utf8Encoder = new TextEncoder("UTF-8");
    let stringBuffer = utf8Encoder.encode(code);
    let len = stringBuffer.length;
    let ptr = this.wasm.alloc(len + 1);

    let memory = new Uint8Array(this.wasm.memory.buffer, ptr);
    for (let i = 0; i < len; i++) {
      memory[i] = stringBuffer[i];
    }

    memory[len] = 0;

    return ptr;
  }

  readStringFromBuffer(ptr) {
    const u8Buffer = new Uint8Array(this.wasm.memory.buffer, ptr);
    const buffer = Uint8Array.from(collectCString(u8Buffer));

    const utf8Decoder = new TextDecoder("UTF-8");
    return utf8Decoder.decode(buffer);
  }
}

function* collectCString(buffer) {
  let ptr = 0;

  while (buffer[ptr] !== 0) {
    if (buffer[ptr] === undefined) {
      throw new Error("Tried to read undef mem");
    }

    yield buffer[ptr];
    ptr += 1;
  }
}

export default function getInterpreter() {
  return fetch(wasmLibPath)
    .then(response => response.arrayBuffer())
    .then(buffer =>
      WebAssembly.instantiate(buffer, {
        env: { get_current_js_time: () => Math.floor(Date.now() / 1000) }
      })
    )
    .then(results => new RloxInterpreter(results.instance.exports));
}
