import { DifferenceCommands } from "./rustTypes"

let path = window.location.host
let paths = window.location.pathname.split("/");
for (let i = 0; i < paths.length - 1; i++) {
  if (paths[i] === "") continue
  path += "/" + paths[i]
}
const websocket = new WebSocket(`ws://${path}/ws`)

function send(obj: any) {
  if (websocket.readyState === WebSocket.OPEN) {
    websocket.send(JSON.stringify(obj))
  }
}

function onopen(callback: () => void) {
  websocket.onopen = callback
}

function oncommand(callback: (cmds: DifferenceCommands) => void) {
  websocket.onmessage = (evt) => {
    const cmds = JSON.parse(evt.data) as DifferenceCommands
    callback(cmds)
  }
}

export default { send, onopen, oncommand }
