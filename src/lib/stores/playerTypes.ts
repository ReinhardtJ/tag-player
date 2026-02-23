export interface Tags {
  title: string | null
  artist: string | null
  album_artist: string | null
  album: string | null
  date: string | null
  genre: string | null
  mood: string | null
  track_number: number | null
}

export interface Song {
  path: string
  name: string
  duration_millis: number
  tags: Map<string, string>
}

export interface Library {
  songs: Song[]
  errors: string[]
}

export interface SongDto {
  path: string
  name: string
  duration_millis: number
  tags: Record<string, string>
}

export interface LibraryDto {
  songs: SongDto[]
  errors: string[]
}