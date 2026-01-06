import { playerState } from '$lib/stores/player.svelte'

class ErrorState {
  // error = $derived(playerState.library.errors.join(',\n'))
  private _error = $state('')
  get error() {
    return this._error
  }

  addError(error: string) {
    this._error = error
  }
}

export const errorState = new ErrorState()
