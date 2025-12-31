interface Tags {
  title: string
  artist: string
  album_artist: string
  album: string
  date: string
  genre: string
  mood: string
  track_number: number
}

interface Song {
  path: string
  name: string
  tags: Tags
}

interface Library {
  songs: Song[]
  errors: string[]
}

class PlayerState {
  library = $state<Library>({ songs: [], errors: []})
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
