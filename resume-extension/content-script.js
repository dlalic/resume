/* global browser, handleResponse, handleError */

function onMouseUp (e) {
  const activeInput = document.activeElement
  if (!activeInput) {
    return
  }
  var sending = browser.runtime.sendMessage({
    id: activeInput.id,
    type: activeInput.type,
    name: activeInput.name
  })
  sending.then(handleResponse, handleError)
}

document.addEventListener('mouseup', onMouseUp)
