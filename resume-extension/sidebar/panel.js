const content = document.querySelector('#content')
const suggestions = document.querySelector('#suggestions')

function handleMessage (request, sender, sendResponse) {
  suggestions.style.display = 'block'
  	content.innerHTML = 'Active element is: id=' + request.id + ' type=' + request.type + ' name=' + request.name
}

browser.runtime.onMessage.addListener(handleMessage)
