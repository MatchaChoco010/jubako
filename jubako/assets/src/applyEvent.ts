import { HandleEvent, VNodeEventType } from "./rustTypes"
import connection from "./connection"
import {
  eventToPayload,
  dragEventToPayload,
  focusEventToPayload,
  inputEventToPayload,
  mouseEventToPayload,
  touchEventToPayload,
  wheelEventToPayload,
  submitEventToPayload,
  pointerEventToPayload,
  keyboardEventToPayload,
  progressEventToPayload,
  animationEventToPayload,
  transitionEventToPayload
} from "./eventToPayload"

// handling events and send them to jubako server.
export function applyEvent(elem: HTMLElement, eventHandle: HandleEvent) {
  const e = elem as any
  const { handle_id, handle_events, handle_prevent_default_events } = eventHandle

  // handle events.
  for (const event of handle_events) {
    // there is no `oncancel`, `focusin`, `focusout` event in HTMLElement
    // so that we need to use `addEventListener` instead of `oncancel` etc.
    // we want to handle resize event not only when window resized
    // but also when element resized, so we use ResizeObserver.
    if (event === "Cancel") {
      if (e.c instanceof Function) {
        elem.removeEventListener('cancel', e.c)
      }
      const oncancel = (e: Event) => {
        const payload = eventToPayload(e)
        connection.send({
          handle_id,
          kind: { type: "Cancel", content: payload }
        })
      }
      elem.addEventListener('cancel', oncancel)
      e.c = oncancel
    } else if (event === "FocusIn") {
      if (e.fi instanceof Function) {
        e.removeEventListener('focusin', e.fi)
      }
      const onfocusin = (e: FocusEvent) => {
        const payload = focusEventToPayload(e)
        connection.send({
          handle_id,
          kind: { type: "FocusIn", content: payload }
        })
      }
      elem.addEventListener("focusin", onfocusin)
      e.fi = onfocusin
    } else if (event === "FocusOut") {
      if (e.fi instanceof Function) {
        e.removeEventListener('focusout', e.fi)
      }
      const onfocusin = (e: FocusEvent) => {
        const payload = focusEventToPayload(e)
        connection.send({
          handle_id,
          kind: { type: "FocusOut", content: payload }
        })
      }
      elem.addEventListener("focusout", onfocusin)
      e.fi = onfocusin
    } else if (event === "Resize") {
      const ro = new ResizeObserver((entries) => {
        for (let entry of entries) {
          let target: any = null
          if (entry.target !== null || entry.target as Element !== null) {
            const targetElem = entry.target as Element
            target = {
              element_name: targetElem.tagName,
              client_height: targetElem.clientHeight,
              client_width: targetElem.clientWidth,
              client_left: targetElem.clientLeft,
              client_top: targetElem.clientTop,
              scroll_height: targetElem.scrollHeight,
              scroll_width: targetElem.scrollWidth,
              scroll_left: targetElem.scrollLeft,
              scroll_top: targetElem.scrollTop,
            }
          }
        connection.send({
          handle_id,
          kind: { type: "Resize", content: target }
        })
        }
      })
      ro.observe(elem)
      e["ro"] = ro
    } else {
      e[`on${event.toLowerCase()}`] = (e: Event) => {
        if (e instanceof DragEvent) {
          const payload = dragEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof FocusEvent) {
          const payload = focusEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof InputEvent) {
          const payload = inputEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof MouseEvent) {
          const payload = mouseEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof TouchEvent) {
          const payload = touchEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof WheelEvent) {
          const payload = wheelEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof SubmitEvent) {
          const payload = submitEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof PointerEvent) {
          const payload = pointerEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof KeyboardEvent) {
          const payload = keyboardEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof ProgressEvent) {
          const payload = progressEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof AnimationEvent) {
          const payload = animationEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof TransitionEvent) {
          const payload = transitionEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else {
          const payload = eventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        }
      }
    }
  }

  // handle prevent default events
  for (const event of handle_prevent_default_events) {
    // same as above, we use `addEventListener` instead of `oncancel` etc.
    // and same as above, we use ResizeObserver to detect resize.
    if (event === "Cancel") {
      if (e.c instanceof Function) {
        elem.removeEventListener('cancel', e.c)
      }
      const oncancel = (e: Event) => {
        e.preventDefault()
        const payload = eventToPayload(e)
        connection.send({
          handle_id,
          kind: { type: "Cancel", content: payload }
        })
      }
      elem.addEventListener('cancel', oncancel)
      e.c = oncancel
    } else if (event === "FocusIn") {
      if (e.fi instanceof Function) {
        e.removeEventListener('focusin', e.fi)
      }
      const onfocusin = (e: FocusEvent) => {
        e.preventDefault()
        const payload = focusEventToPayload(e)
        connection.send({
          handle_id,
          kind: { type: "FocusIn", content: payload }
        })
      }
      elem.addEventListener("focusin", onfocusin)
      e.fi = onfocusin
    } else if (event === "FocusOut") {
      if (e.fi instanceof Function) {
        e.removeEventListener('focusout', e.fi)
      }
      const onfocusin = (e: FocusEvent) => {
        e.preventDefault()
        const payload = focusEventToPayload(e)
        connection.send({
          handle_id,
          kind: { type: "FocusOut", content: payload }
        })
      }
      elem.addEventListener("focusout", onfocusin)
      e.fi = onfocusin
    } else if (event === "Resize") {
      const ro = new ResizeObserver((entries) => {
        for (let entry of entries) {
          let target: any = null
          if (entry.target !== null || entry.target as Element !== null) {
            const targetElem = entry.target as Element
            target = {
              element_name: targetElem.tagName,
              client_height: targetElem.clientHeight,
              client_width: targetElem.clientWidth,
              client_left: targetElem.clientLeft,
              client_top: targetElem.clientTop,
              scroll_height: targetElem.scrollHeight,
              scroll_width: targetElem.scrollWidth,
              scroll_left: targetElem.scrollLeft,
              scroll_top: targetElem.scrollTop,
            }
          }
          connection.send({
            handle_id,
            kind: { type: "Resize", content: target }
          })
        }
      })
      ro.observe(elem)
      e["ro"] = ro
    } else {
      e[`on${event.toLowerCase()}`] = (e: Event) => {
        e.preventDefault()
        if (e instanceof DragEvent) {
          const payload = dragEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof FocusEvent) {
          const payload = focusEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof InputEvent) {
          const payload = inputEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof MouseEvent) {
          const payload = mouseEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof TouchEvent) {
          const payload = touchEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof WheelEvent) {
          const payload = wheelEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof SubmitEvent) {
          const payload = submitEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof PointerEvent) {
          const payload = pointerEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof KeyboardEvent) {
          const payload = keyboardEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof ProgressEvent) {
          const payload = progressEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof AnimationEvent) {
          const payload = animationEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else if (e instanceof TransitionEvent) {
          const payload = transitionEventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        } else {
          const payload = eventToPayload(e)
          connection.send({
            handle_id,
            kind: { type: event, content: payload }
          })
        }
      }
    }
  }

  for (const event of Object.keys(VNodeEventType).filter(
    (key) => !handle_events.includes(key as VNodeEventType) && !handle_prevent_default_events.includes(key as VNodeEventType)
  )) {
    if (event === "Cancel") {
      if (e.c instanceof Function) {
        e.removeEventListener('cancel', e.c)
      }
    } else if (event === "FocusIn") {
      if (e.fi instanceof Function) {
        e.removeEventListener('focusin', e.fi)
      }
    } else if (event === "FocusOn") {
      if (e.fo instanceof Function) {
        e.removeEventListener('focusout', e.fo)
      }
    } else if (event === "Resize") {
      if (e.ro instanceof ResizeObserver) {
        e.ro.disconnect()
      }
    } else {
      e[`on${event.toLowerCase()}`] = null
    }
  }
}
