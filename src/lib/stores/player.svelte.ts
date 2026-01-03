import { invoke } from '@tauri-apps/api/core'

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
  duration_millis: number
  tags: Tags
}

interface Library {
  songs: Song[]
  errors: string[]
}

class PlayerState {
  library = $state<Library>({ songs: [], errors: [] })
  isPlaying = $derived(false)
  isLoaded = $state(false)
  currentSong = $state<Song | null>(null)
  searchQuery = $state('')

  filteredSongs = $derived(
    this.searchQuery.trim() === ''
      ? this.library.songs
      : this.library.songs.filter(song => matchesSearch(song, this.searchQuery))
  )

  reset() {
    this.isPlaying = false
    this.isLoaded = false
    this.currentSong = null
  }

  async seek(positionMillis: number) {
    await invoke('seek', { positionMillis })
  }

  async changeVolume(volumeFrom0To1: number) {
    await invoke('volume_change', { volume: volumeFrom0To1 })
  }
}

function matchesSearch(song: Song, query: string): boolean {
  const searchTerm = query.toLowerCase().trim()

  const searchableFields = [
    song.name,
    song.tags.title,
    song.tags.artist,
    song.tags.album_artist,
    song.tags.album,
    song.tags.genre,
    song.tags.mood,
  ]

  return searchableFields.some(
    field => field?.toLowerCase().includes(searchTerm)
  )
}

export const playerState = new PlayerState()
export type { Song }
