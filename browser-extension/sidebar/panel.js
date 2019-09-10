/* global browser */

// Settings

const clipboardButton = document.getElementById('clipboard')
clipboardButton.addEventListener('click', () => {
  // TODO: implement clipboard setting
})

const suggestionsButton = document.getElementById('suggestions')
suggestionsButton.addEventListener('click', () => {
  // TODO: implement suggestions setting
})

// Suggestions

function handleMessage () {
  // TODO: display suggestion
}

browser.runtime.onMessage.addListener(handleMessage)

// Pagination

var currentEmploymentsPage = 0
var currentEducationsPage = 0

const pagination = [
  { id: 'employment', counter: currentEmploymentsPage },
  { id: 'education', counter: currentEducationsPage }
]

pagination.forEach(function (element) {
  const previousButton = document.getElementById(element.id + '__previous')
  previousButton.addEventListener('click', (event) => {
    event.preventDefault()
    element.counter--
    element.counter = changePage(element.id, element.counter)
  })

  const nextButton = document.getElementById(element.id + '__next')
  nextButton.addEventListener('click', (event) => {
    event.preventDefault()
    element.counter++
    element.counter = changePage(element.id, element.counter)
  })
})

function changePage (elementClass, page) {
  const elements = document.getElementsByClassName(elementClass)
  if (page < 0) {
    page = 0
  } else if (page > elements.length - 1) {
    page = elements.length - 1
  }
  for (var i = 0; i < elements.length; i++) {
    const element = document.getElementById(elementClass + '__' + i)
    if (element) {
      element.classList.add('d-none')
    }
  }
  const element = document.getElementById(elementClass + '__' + page)
  if (element) {
    element.classList.remove('d-none')
  }
  return page
}

window.addEventListener('load', () => {
  pagination.forEach(function (element) {
    changePage(element.id, element.counter)
  })
  // TODO: load default settings
})

// Clipboard

function copy (event) {
  if (event.target.tagName === 'TEXTAREA') {
    event.target.select()
    document.execCommand('copy')
  }
}

document.addEventListener('click', copy)
