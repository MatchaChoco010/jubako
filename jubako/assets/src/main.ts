import { DifferenceCommands } from "./rustTypes";
import connection from "./connection";
import { applyCommand, applyStyleCommand } from "./applyCommand";

const main = async () => {
  const update = () => {
    connection.send("DRAW")
    requestAnimationFrame(update)
  }

  connection.onopen(() => {
    update()
  })

  const appRoot = document.querySelector<HTMLDivElement>('#app')
  const portalRoot = document.querySelector<HTMLDivElement>('#portal')

  connection.oncommand((cmds: DifferenceCommands) => {
    if (appRoot === null || portalRoot === null) return

    for (const cmd of cmds.main) {
      applyCommand(appRoot, cmd)
    }
    for (const cmd of cmds.portals) {
      applyCommand(portalRoot, cmd)
    }
    for (const cmd of cmds.styles) {
      applyStyleCommand(cmd)
    }
  })
}

document.addEventListener('DOMContentLoaded', main)
