import {
  Element as ElementPayload,
  Event as EventPayload,
  MouseEvent as MouseEventPayload,
  FocusEvent as FocusEventPayload,
  DragEvent as DragEventPayload,
  InputEvent as InputEventPayload,
  KeyboardEvent as KeyboardEventPayload,
  ProgressEvent as ProgressEventPayload,
  SubmitEvent as SubmitEventPayload,
  WheelEvent as WheelEventPayload,
  AnimationEvent as AnimationEventPayload,
  TouchEvent as TouchEventPayload,
  PointerEvent as PointerEventPayload,
  TransitionEvent as TransitionEventPayload,
} from "./rustTypes"

export function eventToPayload(e: Event): EventPayload {
  if (e.target !== null && e.target as Element) {
    const target = e.target as Element
    return {
      target: {
        tag_name: target.tagName,
        client_height: target.clientHeight,
        client_width: target.clientWidth,
        client_left: target.clientLeft,
        client_top: target.clientTop,
        scroll_height: target.scrollHeight,
        scroll_width: target.scrollWidth,
        scroll_left: target.scrollLeft,
        scroll_top: target.scrollTop,
      }
    }
  } else {
    return {
      target: undefined,
    }
  }
}

export function mouseEventToPayload(e: MouseEvent): MouseEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  let related_target: ElementPayload | undefined = undefined
  if (e.relatedTarget !== null && e.relatedTarget as Element !== null) {
    const relatedTargetElem = e.relatedTarget as Element
    related_target = {
      tag_name: relatedTargetElem.tagName,
      client_height: relatedTargetElem.clientHeight,
      client_width: relatedTargetElem.clientWidth,
      client_left: relatedTargetElem.clientLeft,
      client_top: relatedTargetElem.clientTop,
      scroll_height: relatedTargetElem.scrollHeight,
      scroll_width: relatedTargetElem.scrollWidth,
      scroll_left: relatedTargetElem.scrollLeft,
      scroll_top: relatedTargetElem.scrollTop,
    }
  }
  return {
    alt_key: e.altKey,
    button: e.button,
    buttons: e.buttons,
    client_x: e.clientX,
    client_y: e.clientY,
    ctrl_key: e.ctrlKey,
    meta_key: e.metaKey,
    movement_x: e.movementX,
    movement_y: e.movementY,
    offset_x: e.offsetX,
    offset_y: e.offsetY,
    page_x: e.pageX,
    page_y: e.pageY,
    related_target,
    screen_x: e.screenX,
    screen_y: e.screenY,
    shift_key: e.shiftKey,
    target,
    x: e.x,
    y: e.y,
  }
}

export function focusEventToPayload(e: FocusEvent): FocusEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  let related_target: ElementPayload | undefined = undefined
  if (e.relatedTarget !== null && e.relatedTarget as Element !== null) {
    const relatedTargetElem = e.relatedTarget as Element
    related_target = {
      tag_name: relatedTargetElem.tagName,
      client_height: relatedTargetElem.clientHeight,
      client_width: relatedTargetElem.clientWidth,
      client_left: relatedTargetElem.clientLeft,
      client_top: relatedTargetElem.clientTop,
      scroll_height: relatedTargetElem.scrollHeight,
      scroll_width: relatedTargetElem.scrollWidth,
      scroll_left: relatedTargetElem.scrollLeft,
      scroll_top: relatedTargetElem.scrollTop,
    }
  }
  return { related_target, target }
}

export function dragEventToPayload(e: DragEvent): DragEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  let related_target: ElementPayload | undefined = undefined
  if (e.relatedTarget !== null && e.relatedTarget as Element !== null) {
    const relatedTargetElem = e.relatedTarget as Element
    related_target = {
      tag_name: relatedTargetElem.tagName,
      client_height: relatedTargetElem.clientHeight,
      client_width: relatedTargetElem.clientWidth,
      client_left: relatedTargetElem.clientLeft,
      client_top: relatedTargetElem.clientTop,
      scroll_height: relatedTargetElem.scrollHeight,
      scroll_width: relatedTargetElem.scrollWidth,
      scroll_left: relatedTargetElem.scrollLeft,
      scroll_top: relatedTargetElem.scrollTop,
    }
  }
  return {
    alt_key: e.altKey,
    button: e.button,
    buttons: e.buttons,
    client_x: e.clientX,
    client_y: e.clientY,
    ctrl_key: e.ctrlKey,
    meta_key: e.metaKey,
    movement_x: e.movementX,
    movement_y: e.movementY,
    offset_x: e.offsetX,
    offset_y: e.offsetY,
    page_x: e.pageX,
    page_y: e.pageY,
    related_target,
    screen_x: e.screenX,
    screen_y: e.screenY,
    shift_key: e.shiftKey,
    target,
    x: e.x,
    y: e.y,
  }
}

export function inputEventToPayload(e: InputEvent): InputEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  return {
    data: e.data ?? "",
    input_type: e.inputType,
    is_composing: e.isComposing,
    target
  }
}

export function keyboardEventToPayload(e: KeyboardEvent): KeyboardEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  return {
    alt_key: e.altKey,
    char_code: e.charCode,
    code: e.code,
    ctrl_key: e.ctrlKey,
    key: e.key,
    key_code: e.keyCode,
    location: e.location,
    meta_key: e.metaKey,
    repeat: e.repeat,
    shift_key: e.shiftKey,
    target,
  }
}

export function progressEventToPayload(e: ProgressEvent): ProgressEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  return {
    length_computable: e.lengthComputable,
    loaded: e.loaded,
    target,
    total: e.total,
  }
}

export function submitEventToPayload(e: SubmitEvent): SubmitEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  let submitter: ElementPayload | undefined = undefined
  if (e.submitter !== null && e.submitter as Element !== null) {
    const submitter_elem = e.submitter as Element
    submitter = {
      tag_name: submitter_elem.tagName,
      client_height: submitter_elem.clientHeight,
      client_width: submitter_elem.clientWidth,
      client_left: submitter_elem.clientLeft,
      client_top: submitter_elem.clientTop,
      scroll_height: submitter_elem.scrollHeight,
      scroll_width: submitter_elem.scrollWidth,
      scroll_left: submitter_elem.scrollLeft,
      scroll_top: submitter_elem.scrollTop,
    }
  }
  return { submitter, target }
}

export function wheelEventToPayload(e: WheelEvent): WheelEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  return {
    alt_key: e.altKey,
    button: e.button,
    buttons: e.buttons,
    client_x: e.clientX,
    client_y: e.clientY,
    ctrl_key: e.ctrlKey,
    delta_mode: e.deltaMode,
    delta_x: e.deltaX,
    delta_y: e.deltaY,
    delta_z: e.deltaZ,
    meta_key: e.metaKey,
    movement_x: e.movementX,
    movement_y: e.movementY,
    offset_x: e.offsetX,
    offset_y: e.offsetY,
    page_x: e.pageX,
    page_y: e.pageY,
    screen_x: e.screenX,
    screen_y: e.screenY,
    shift_key: e.shiftKey,
    x: e.x,
    y: e.y,
    target,
  }
}

export function animationEventToPayload(e: AnimationEvent): AnimationEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  return {
    animation_name: e.animationName,
    elapsed_time: e.elapsedTime,
    pseudo_element: e.pseudoElement,
    target,
  }
}

export function pointerEventToPayload(e: PointerEvent): PointerEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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

  let related_target: ElementPayload | undefined = undefined
  if (e.relatedTarget !== null && e.relatedTarget as Element !== null) {
    const relatedTargetElem = e.relatedTarget as Element
    related_target = {
      tag_name: relatedTargetElem.tagName,
      client_height: relatedTargetElem.clientHeight,
      client_width: relatedTargetElem.clientWidth,
      client_left: relatedTargetElem.clientLeft,
      client_top: relatedTargetElem.clientTop,
      scroll_height: relatedTargetElem.scrollHeight,
      scroll_width: relatedTargetElem.scrollWidth,
      scroll_left: relatedTargetElem.scrollLeft,
      scroll_top: relatedTargetElem.scrollTop,
    }
  }
  return {
    alt_key: e.altKey,
    button: e.button,
    buttons: e.buttons,
    client_x: e.clientX,
    client_y: e.clientY,
    ctrl_key: e.ctrlKey,
    height: e.height,
    is_primary: e.isPrimary,
    meta_key: e.metaKey,
    movement_x: e.movementX,
    movement_y: e.movementY,
    offset_x: e.offsetX,
    offset_y: e.offsetY,
    page_x: e.pageX,
    page_y: e.pageY,
    pointer_id: e.pointerId,
    pointer_type: e.pointerType,
    pressure: e.pressure,
    related_target,
    screen_x: e.screenX,
    screen_y: e.screenY,
    shift_key: e.shiftKey,
    target,
    tilt_x: e.tiltX,
    tilt_y: e.tiltY,
    twist: e.twist,
    width: e.width,
    x: e.x,
    y: e.y,
  }
}

export function touchEventToPayload(e: TouchEvent): TouchEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  const changed_touches = []
  for (let i = 0; i < e.changedTouches.length; i++) {
    const touch = e.changedTouches[i]
    let touchTarget: ElementPayload | undefined = undefined
    if (touch.target !== null && touch.target as Element !== null) {
      const touchTargetElem = touch.target as Element
      touchTarget = {
        tag_name: touchTargetElem.tagName,
        client_height: touchTargetElem.clientHeight,
        client_width: touchTargetElem.clientWidth,
        client_left: touchTargetElem.clientLeft,
        client_top: touchTargetElem.clientTop,
        scroll_height: touchTargetElem.scrollHeight,
        scroll_width: touchTargetElem.scrollWidth,
        scroll_left: touchTargetElem.scrollLeft,
        scroll_top: touchTargetElem.scrollTop,
      }
    }
    changed_touches.push({
      identifier: touch.identifier,
      client_x: touch.clientX,
      client_y: touch.clientY,
      page_x: touch.pageX,
      page_y: touch.pageY,
      radius_x: touch.radiusX,
      radius_y: touch.radiusY,
      rotation_angle: touch.rotationAngle,
      screen_x: touch.screenX,
      screen_y: touch.screenY,
      target: touchTarget,
    })
  }
  const target_touches = []
  for (let i = 0; i < e.targetTouches.length; i++) {
    const touch = e.targetTouches[i]
    let touchTarget: ElementPayload | undefined = undefined
    if (touch.target !== null && touch.target as Element !== null) {
      const touchTargetElem = touch.target as Element
      touchTarget = {
        tag_name: touchTargetElem.tagName,
        client_height: touchTargetElem.clientHeight,
        client_width: touchTargetElem.clientWidth,
        client_left: touchTargetElem.clientLeft,
        client_top: touchTargetElem.clientTop,
        scroll_height: touchTargetElem.scrollHeight,
        scroll_width: touchTargetElem.scrollWidth,
        scroll_left: touchTargetElem.scrollLeft,
        scroll_top: touchTargetElem.scrollTop,
      }
    }
    target_touches.push({
      identifier: touch.identifier,
      client_x: touch.clientX,
      client_y: touch.clientY,
      page_x: touch.pageX,
      page_y: touch.pageY,
      radius_x: touch.radiusX,
      radius_y: touch.radiusY,
      rotation_angle: touch.rotationAngle,
      screen_x: touch.screenX,
      screen_y: touch.screenY,
      target: touchTarget,
    })
  }
  const touches = []
  for (let i = 0; i < e.touches.length; i++) {
    const touch = e.touches[i]
    let touchTarget: ElementPayload | undefined = undefined
    if (touch.target !== null && touch.target as Element !== null) {
      const touchTargetElem = touch.target as Element
      touchTarget = {
        tag_name: touchTargetElem.tagName,
        client_height: touchTargetElem.clientHeight,
        client_width: touchTargetElem.clientWidth,
        client_left: touchTargetElem.clientLeft,
        client_top: touchTargetElem.clientTop,
        scroll_height: touchTargetElem.scrollHeight,
        scroll_width: touchTargetElem.scrollWidth,
        scroll_left: touchTargetElem.scrollLeft,
        scroll_top: touchTargetElem.scrollTop,
      }
    }
    touches.push({
      identifier: touch.identifier,
      client_x: touch.clientX,
      client_y: touch.clientY,
      page_x: touch.pageX,
      page_y: touch.pageY,
      radius_x: touch.radiusX,
      radius_y: touch.radiusY,
      rotation_angle: touch.rotationAngle,
      screen_x: touch.screenX,
      screen_y: touch.screenY,
      target: touchTarget,
    })
  }
  return {
    alt_key: e.altKey,
    changed_touches,
    ctrl_key: e.ctrlKey,
    meta_key: e.metaKey,
    shift_key: e.shiftKey,
    target_touches,
    touches,
    target,
  }
}

export function transitionEventToPayload(e: TransitionEvent): TransitionEventPayload {
  let target: ElementPayload | undefined = undefined
  if (e.target !== null && e.target as Element !== null) {
    const targetElem = e.target as Element
    target = {
        tag_name: targetElem.tagName,
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
  return {
    elapsed_time: e.elapsedTime,
    property_name: e.propertyName,
    pseudo_element: e.pseudoElement,
    target,
  }
}
