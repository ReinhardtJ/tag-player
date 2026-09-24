export interface MusicBrainzArtist {
  id: string
  name: string
  'sort-name'?: string | null
}

export interface MusicBrainzArtistCredit {
  name: string
  artist: MusicBrainzArtist
  joinphrase?: string | null
}

export interface MusicBrainzReleaseGroup {
  id: string
  'first-release-date'?: string | null
}

export interface MusicBrainzRelease {
  id: string
  title: string
  date?: string | null
  country?: string | null
  status?: string | null
  disambiguation?: string | null
  'artist-credit'?: MusicBrainzArtistCredit[] | null
  'release-group'?: MusicBrainzReleaseGroup | null
}

export interface MusicBrainzRecording {
  id: string
  title: string
  disambiguation?: string | null
  'first-release-date'?: string | null
  'artist-credit'?: MusicBrainzArtistCredit[] | null
  releases?: MusicBrainzRelease[] | null
}

export function formatArtistCredit(recording: MusicBrainzRecording): string {
  const artistCredit = recording['artist-credit'] ?? []
  return artistCredit.map((credit) => credit.name).join(', ')
}

export function formatRelease(recording: MusicBrainzRecording): string {
  const release = recording.releases?.[0]
  if (!release)
    return ''

  const year = release.date?.slice(0, 4)
  return year ? `${release.title} (${year})` : release.title
}
