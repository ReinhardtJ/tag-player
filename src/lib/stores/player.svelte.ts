interface MusicFile {
  path: string
  name: string
}

class PlayerState {
  songList = $state<MusicFile[]>([])
  isPlaying = $state(false)
  isLoaded = $state(false)

  reset() {
    this.isPlaying = false
    this.isLoaded = false
  }
}

export const playerState = new PlayerState()
export type { MusicFile }
