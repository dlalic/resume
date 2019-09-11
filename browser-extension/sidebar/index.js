import("../pkg/index.js").then(wasm => {
  function handleMessage () {
    // TODO: bind to wasm
  }
  browser.runtime.onMessage.addListener(handleMessage)
}).catch(console.error);
