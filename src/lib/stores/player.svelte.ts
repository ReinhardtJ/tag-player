interface Song {
  path: string
  name: string
}

class PlayerState {
  songList = $state<Song[]>([])
  isPlaying = $derived(false)
  isLoaded = $state(false)
  currentSong = $state<Song | null>(null)

  reset() {
    this.isPlaying = false
    this.isLoaded = false
    this.currentSong = null
  }
}

export const playerState = new PlayerState()
export type { Song }
