export class DialogStore {
  private _open = $state(false)

  get open() {
    return this._open
  }

  show() {
    this._open = true
  }

  close() {
    this._open = false
  }

  toggle() {
    this._open = !this._open
  }
}

let dialogStore: DialogStore | undefined = undefined

export function useDialogStore() {
  if (dialogStore === undefined) {
    dialogStore = new DialogStore()
  }
  return dialogStore
}