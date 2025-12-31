interface MusicFile {
  path: string
  name: string
}

class PlayerState {
  songList = $state<MusicFile[]>([])
  isPlaying = $derived(false)
  isLoaded = $state(false)
  currentSong = $state<MusicFile | null>(null)

  reset() {
    this.isPlaying = false
    this.isLoaded = false
    this.currentSong = null
  }
}

export const playerState = new PlayerState()
export type { MusicFile }
