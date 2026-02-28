import { invoke } from "@tauri-apps/api/core"
import { type ErrorStore, useErrorStore } from "./errorStore.svelte"
import {
  type TagEditorStore,
  useTagEditorStore,
} from "./tagEditorStore.svelte.ts"
import type {
  Library,
  LibraryDto,
  Song,
  SongDto,
} from "$lib/stores/playerTypes.ts"

function dto_to_song(dto: SongDto): Song {
  return {
    path: dto.path,
    name: dto.name,
    duration_millis: dto.duration_millis,
    tags: new Map<string, string>(Object.entries(dto.tags)),
    cover_base64: dto.cover_base64,
  }
}

function dto_to_library(dto: LibraryDto): Library {
  return {
    songs: dto.songs.map(dto_to_song),
    errors: dto.errors,
  }
}

export class PlayerStore {
  library = $state<Library>({ songs: [], errors: [] })
  isPlaying = $derived(false)
  isLoaded = $state(false)
  currentSong = $state<Song | null>(null)
  searchQuery = $state("")
  positionMillis = $state(0)
  isSeeking = $state(false)

  private errorStore: ErrorStore = useErrorStore()
  private tagEditorStore: TagEditorStore = useTagEditorStore()

  constructor() {}

  filteredSongs = $derived(
    this.searchQuery.trim() === ""
      ? this.library.songs
      : this.library.songs.filter((song) =>
        matchesSearch(song, this.searchQuery),
      ),
  )

  reset() {
    this.isPlaying = false
    this.isLoaded = false
    this.currentSong = null
    this.positionMillis = 0
  }

  async seek() {
    try {
      this.isSeeking = true
      await invoke("seek", { positionMillis: this.positionMillis })
    } finally {
      // pause updates for another 100ms to avoid jumping while
      // position update thread is still aligning to decoder thread
      setTimeout(() => (this.isSeeking = false), 100)
    }
  }

  async loadMusicLibrary(libraryPath: string) {
    const library = (await invoke("get_music_library", {
      path: libraryPath,
    })) as LibraryDto
    for (const error of library.errors) {
      this.errorStore.addError(error)
    }
    this.library = dto_to_library(library)
  }

  async changeVolume(volumeFrom0To1: number) {
    await invoke("volume_change", { volume: volumeFrom0To1 })
  }

  async play(song: Song) {
    try {
      const result = await invoke("load_and_play", { path: song.path })
      if (result !== null) {
        this.errorStore.addError(String(result))
        return
      }
      this.reset()
      this.isLoaded = true
      this.isPlaying = true
      this.currentSong = song
      this.tagEditorStore.setTags(song.tags)
    } catch (e) {
      this.errorStore.addError(String(e))
    }
  }
}

function matchesSearch(song: Song, query: string): boolean {
  const searchTerm = query.toLowerCase().trim()
  const searchableFields = song.tags.values().toArray()
  return searchableFields.some((field) =>
    field?.toLowerCase().includes(searchTerm),
  )
}

let playerStore: PlayerStore | undefined = undefined

export function usePlayerStore() {
  if (playerStore === undefined) {
    playerStore = new PlayerStore()
  }
  return playerStore
}
