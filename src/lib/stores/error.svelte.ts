import { playerState } from '$lib/stores/player.svelte'

class ErrorState {
  error = $derived(playerState.library.errors.join(',\n'))
}

export const errorState = new ErrorState()
