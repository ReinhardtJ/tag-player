class ErrorState {
  private _errors = $state<string[]>([])
  get errors() {
    return this._errors
  }

  addError(error: string) {
    this._errors.push(error)
  }
}

export const errorState = new ErrorState()
