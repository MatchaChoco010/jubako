import { DifferenceCommand, StyleDifferenceCommand } from "./rustTypes"
import { applyEvent } from "./applyEvent"

// apply VNode difference command to DOM
export function applyCommand(root: Node, cmd: DifferenceCommand) {
  if (cmd.type === "UpdateElement") {
    const { index, class_diff, props_diff, event_diff, children } = cmd.content
    const element = root.childNodes[index]
    if (element === undefined) return

    if (element instanceof HTMLElement) {
      // update class
      for (const diff of class_diff) {
        if (diff.type === "Add") {
          element.classList.add(diff.content)
        } else if (diff.type === "Remove") {
          element.classList.remove(diff.content)
        }
      }

      // update props
      for (const diff of props_diff) {
        if (diff.type === "Add") {
          if (diff.content.split("=").length === 2) {
            const [key, value] = diff.content.split("=")
            element.setAttribute(key, value)
          } else {
            element.setAttribute(diff.content, "true")
          }
        } else if (diff.type === "Remove") {
          if (diff.content.split("=").length === 2) {
            const [key, _value] = diff.content.split("=")
            element.removeAttribute(key)
          } else {
            element.removeAttribute(diff.content)
          }
        }
      }

      // update events
      if (event_diff.type === "Update") {
        const event = event_diff.content
        applyEvent(element, event)
      }
    }

    for (const child_cmd of children) {
      applyCommand(element, child_cmd)
    }
  } else if (cmd.type === "UpdateText") {
    const { index, new_text } = cmd.content
    const text = root.childNodes[index]
    if (text === undefined) return
    text.textContent = new_text
  } else if (cmd.type === "ReplaceToElement") {
    const { index, new_tag, classes, props, event, children } = cmd.content
    const element = root.childNodes[index]
    if (element === undefined) return

    // create new element to replace
    const new_element = document.createElement(new_tag)

    // add classes
    new_element.classList.add(...classes)

    // add props
    for (const prop of props) {
      if (prop.split("=").length === 2) {
        const [key, value] = prop.split("=")
        new_element.setAttribute(key, value)
      } else {
        new_element.setAttribute(prop, "true")
      }
    }

    // add events
    applyEvent(new_element, event)

    for (const child_cmd of children) {
      applyCommand(new_element, child_cmd)
    }

    // replace element with new element
    root.replaceChild(new_element, element)
  } else if (cmd.type === "ReplaceToText") {
    const { index, text } = cmd.content
    const element = root.childNodes[index]
    if (element === undefined) return

    // create new text to replace
    const new_text = document.createTextNode(text)

    // replace element with new text
    root.replaceChild(new_text, element)
  } else if (cmd.type === "InsertElement") {
    const { index, tag, classes, props, event, children } = cmd.content

    // create new element
    const new_element = document.createElement(tag)

    // add classes
    new_element.classList.add(...classes)

    // add props
    for (const prop of props) {
      if (prop.split("=").length === 2) {
        const [key, value] = prop.split("=")
        new_element.setAttribute(key, value)
      } else {
        new_element.setAttribute(prop, "true")
      }
    }

    // add events
    applyEvent(new_element, event)

    for (const child_cmd of children) {
      applyCommand(new_element, child_cmd)
    }

    // insert element
    root.insertBefore(new_element, root.childNodes[index])
  } else if (cmd.type === "InsertText") {
    const { index, text } = cmd.content

    // create new text
    const new_text = document.createTextNode(text)

    // insert text
    root.insertBefore(new_text, root.childNodes[index])
  } else if (cmd.type === "Remove") {
    const { index } = cmd.content
    const element = root.childNodes[index]
    if (element === undefined) return
    root.removeChild(element)
  }
}

// apply style difference command to DOM
export function applyStyleCommand(cmd: StyleDifferenceCommand) {
  if (cmd.type === "AddStyle") {
    const { class_name, value } = cmd.content
    const head = document.head || document.getElementsByTagName("head")[0]
    const style = document.createElement("style")

    style.id = class_name
    style.appendChild(document.createTextNode(value))

    head.appendChild(style)
  } else if (cmd.type === "RemoveStyle") {
    const { class_name } = cmd.content
    document.getElementById(class_name)?.remove()
  }
}
