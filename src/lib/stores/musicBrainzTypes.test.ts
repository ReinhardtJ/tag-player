import { describe, it, expect } from 'vitest'
import {
  formatArtistCredit,
  formatRelease,
  type MusicBrainzRecording
} from './musicBrainzTypes.ts'

function buildRecording(overrides: Partial<MusicBrainzRecording> = {}): MusicBrainzRecording {
  return { id: 'recording-id', title: 'Song', ...overrides }
}

describe('formatArtistCredit', () => {
  it('joins all credited artists with commas', () => {
    const recording = buildRecording({
      'artist-credit': [
        { name: 'Alice', artist: { id: 'artist-1', name: 'Alice' } },
        { name: 'Bob', artist: { id: 'artist-2', name: 'Bob' } }
      ]
    })

    expect(formatArtistCredit(recording)).toBe('Alice, Bob')
  })

  it('returns an empty string when there is no artist credit', () => {
    expect(formatArtistCredit(buildRecording())).toBe('')
  })
})

describe('formatRelease', () => {
  it('uses the first release title and its year', () => {
    const recording = buildRecording({
      releases: [{ id: 'release-id', title: 'Nevermind', date: '1991-09-24' }]
    })

    expect(formatRelease(recording)).toBe('Nevermind (1991)')
  })

  it('omits the year when the release has no date', () => {
    const recording = buildRecording({
      releases: [{ id: 'release-id', title: 'Nevermind' }]
    })

    expect(formatRelease(recording)).toBe('Nevermind')
  })

  it('returns an empty string when there are no releases', () => {
    expect(formatRelease(buildRecording())).toBe('')
  })
})
