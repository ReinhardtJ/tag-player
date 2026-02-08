export class ErrorState {
  private _errors = $state<string[]>([])
  get errors() {
    return this._errors
  }

  addError(error: string) {
    this._errors.push(error)
  }
}

let errorState: ErrorState | undefined = undefined
export function useErrorState() {
  if (errorState === undefined) {
    errorState = new ErrorState()
  }
  return errorState
}
