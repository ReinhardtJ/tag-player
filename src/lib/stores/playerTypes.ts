export interface Song {
  path: string
  name: string
  duration_millis: number
  tags: Map<string, string>
  cover_base64: string | null
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
  cover_base64: string | null
}

export interface LibraryDto {
  songs: SongDto[]
  errors: string[]
}
