/* global browser */

// Settings

const clipboardButton = document.getElementById('clipboard')
const suggestionsButton = document.getElementById('suggestions')

// Pagination

const pagination = [
  { class: 'employment', counter: 0 },
  { class: 'education', counter: 0 }
]

pagination.forEach(function (paginatedElement) {
  const previousButton = document.getElementById(paginatedElement.class + '__previous')
  previousButton.addEventListener('click', (event) => {
    event.preventDefault()
    paginatedElement.counter--
    paginatedElement.counter = changePage(paginatedElement.class, paginatedElement.counter)
  })

  const nextButton = document.getElementById(paginatedElement.class + '__next')
  nextButton.addEventListener('click', (event) => {
    event.preventDefault()
    paginatedElement.counter++
    paginatedElement.counter = changePage(paginatedElement.class, paginatedElement.counter)
  })
})

function changePage (elementClass, page) {
  const elements = document.getElementsByClassName(elementClass)
  page = Math.max(page, 0)
  page = Math.min(page, elements.length - 1)
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
    changePage(element.class, element.counter)
  })
  clipboardButton.checked = true
  suggestionsButton.checked = true
})

// Clipboard

function copy (event) {
  if (event.target.tagName === 'TEXTAREA' && clipboardButton.checked) {
    event.target.select()
    document.execCommand('copy')
  }
}

document.addEventListener('click', copy)

// Suggestions

function handleMessage (message) {
  if (!suggestionsButton.checked) {
    return
  }
  const textAreas = [...document.getElementsByTagName('textarea')]
  textAreas.forEach((textArea) => {
    textArea.classList.remove('is-valid')
    // TODO: investigate and implement possible values other than 'company' etc
    const fieldType = textArea.id.split('__')[0]
    if (!fieldType) {
      return
    }
    for (const [, value] of Object.entries(message)) {
      if (value.includes(fieldType)) {
        const elementClass = textArea.parentNode.parentNode.className
        const paginatedElement = pagination.find(o => o.class === elementClass)
        const current = paginatedElement.counter
        const element = document.getElementById(fieldType + '__' + current)
        element.classList.add('is-valid')
        break
      }
    }
  })
}

browser.runtime.onMessage.addListener(handleMessage)
