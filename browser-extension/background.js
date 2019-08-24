var port = browser.runtime.connectNative("resume");

port.onMessage.addListener((response) => {
  console.log("Received:");
  console.log(response);
});

function handleMessage (request, sender, sendResponse) {
  console.log("Sending:");
  console.log(request);
  port.postMessage(request);
}

browser.runtime.onMessage.addListener(handleMessage);
