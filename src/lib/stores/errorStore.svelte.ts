export class ErrorStore {
  private _errors = $state<string[]>([])
  get errors() {
    return this._errors
  }

  addError(error: string) {
    this._errors.push(error)
  }
}

let errorStore: ErrorStore | undefined = undefined
export function useErrorStore() {
  if (errorStore === undefined) {
    errorStore = new ErrorStore()
  }
  return errorStore
}
