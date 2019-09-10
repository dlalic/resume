/* global browser */

const content = document.querySelector('#content')
const suggestions = document.querySelector('#suggestions')

function handleMessage (request, sender, sendResponse) {
  suggestions.style.display = 'block'
  content.innerHTML = 'Active element is: id=' + request.id + ' type=' + request.type + ' name=' + request.name
}

browser.runtime.onMessage.addListener(handleMessage)

function copy(event) {
  if (event.target.tagName == 'TEXTAREA') {
    var copyText = event.target;
    copyText.select();
    document.execCommand("copy");
  }
}

document.addEventListener("click", copy);