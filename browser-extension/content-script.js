/* global browser, handleResponse, handleError */

function onMouseUp () {
  const activeInput = document.activeElement
  const inputs = ['INPUT', 'TEXTAREA'];
  if (!inputs.includes(activeInput.tagName)) {
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
