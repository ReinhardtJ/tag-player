class ErrorState {
  error = $state('')

  setError(message: string) {
    this.error = message
  }

  clearError() {
    this.error = ''
  }
}

export const errorState = new ErrorState()
