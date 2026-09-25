import { describe, it, expect, vi, beforeEach } from 'vitest'
import { SvelteMap } from 'svelte/reactivity'

const { invokeMock } = vi.hoisted(() => ({ invokeMock: vi.fn() }))
vi.mock('@tauri-apps/api/core', () => ({ invoke: invokeMock }))

import { TagEditorStore } from './tagEditorStore.svelte.ts'
import type { Song } from './playerTypes.ts'

function createSong(overrides: Partial<Song> = {}): Song {
  return {
    path: '/music/song.mp3',
    name: 'song.mp3',
    duration_millis: 1000,
    tags: new SvelteMap<string, string>([['TrackTitle', 'A Song']]),
    cover_base64: null,
    ...overrides
  }
}

// Every command resolves to a sensible default; read_cover_data_url returns the
// given URL so tests can assert what the store writes back onto the song.
function mockInvoke(readCoverDataUrl: string | null = null) {
  invokeMock.mockImplementation(async (command: string) => {
    if (command === 'get_supported_tags') 
      return []
    if (command === 'read_cover_data_url') 
      return readCoverDataUrl
    return null
  })
}

describe('TagEditorStore cover editing', () => {
  beforeEach(() => {
    invokeMock.mockReset()
    mockInvoke()
  })

  describe('cover state', () => {
    it('setPendingCover stores the path and clears the removed flag', () => {
      const store = new TagEditorStore()
      store.removeCover()

      store.setPendingCover('/pictures/cover.png')

      expect(store.pendingCoverPath).toBe('/pictures/cover.png')
      expect(store.isCoverRemoved).toBe(false)
    })

    it('removeCover clears the pending path and marks the cover removed', () => {
      const store = new TagEditorStore()
      store.setPendingCover('/pictures/cover.png')

      store.removeCover()

      expect(store.pendingCoverPath).toBeNull()
      expect(store.isCoverRemoved).toBe(true)
    })

    it('restoreCover clears the removed flag', () => {
      const store = new TagEditorStore()
      store.removeCover()

      store.restoreCover()

      expect(store.isCoverRemoved).toBe(false)
    })

    it('setTags resets any pending cover change', () => {
      const store = new TagEditorStore()
      store.setPendingCover('/pictures/cover.png')

      store.setTags(new SvelteMap([['TrackTitle', 'A Song']]))

      expect(store.pendingCoverPath).toBeNull()
      expect(store.isCoverRemoved).toBe(false)
    })
  })

  describe('saveTags', () => {
    it('writes and reads back the cover when a new cover is pending', async () => {
      const store = new TagEditorStore()
      const song = createSong()
      mockInvoke('data:image/png;base64,NEW')
      store.setTags(song.tags)
      store.setPendingCover('/pictures/cover.png')

      await store.saveTags(song)

      expect(invokeMock).toHaveBeenCalledWith('write_cover', {
        path: song.path,
        coverPath: '/pictures/cover.png'
      })
      expect(invokeMock).toHaveBeenCalledWith('read_cover_data_url', { path: song.path })
      expect(song.cover_base64).toBe('data:image/png;base64,NEW')
      expect(store.pendingCoverPath).toBeNull()
    })

    it('removes the cover and reads back the now-empty cover', async () => {
      const store = new TagEditorStore()
      const song = createSong({ cover_base64: 'data:image/png;base64,OLD' })
      mockInvoke(null)
      store.setTags(song.tags)
      store.removeCover()

      await store.saveTags(song)

      expect(invokeMock).toHaveBeenCalledWith('write_cover', {
        path: song.path,
        coverPath: null
      })
      expect(invokeMock).toHaveBeenCalledWith('read_cover_data_url', { path: song.path })
      expect(song.cover_base64).toBeNull()
      expect(store.isCoverRemoved).toBe(false)
    })

    it('leaves the cover untouched when it was not changed', async () => {
      const store = new TagEditorStore()
      const song = createSong({ cover_base64: 'data:image/png;base64,OLD' })
      store.setTags(song.tags)

      await store.saveTags(song)

      expect(invokeMock).not.toHaveBeenCalledWith('write_cover', expect.anything())
      expect(invokeMock).not.toHaveBeenCalledWith('read_cover_data_url', expect.anything())
      expect(song.cover_base64).toBe('data:image/png;base64,OLD')
    })
  })
})
