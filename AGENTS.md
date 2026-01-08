# AGENTS.md

Guide for AI agents working in the tag-player codebase.

## Project Overview

**Type**: Tauri v2 desktop application with SvelteKit frontend  
**Purpose**: Music player with tag viewing/editing capabilities  
**Stack**:
- Frontend: SvelteKit + Svelte 5 + TypeScript + Tailwind CSS v4
- Backend: Rust (Tauri v2)
- Audio: rodio (playback), lofty (tag reading)
- Build: Vite

**Architecture**: Hybrid app where the frontend (SvelteKit) communicates with Rust backend via Tauri's invoke system. The frontend is prerendered as a static site (SSG) since Tauri doesn't support SSR.

## Essential Commands

### Development
```bash
npm run dev              # Start Vite dev server (frontend only)
npm run tauri dev        # Run full Tauri app in dev mode (recommended)
```

### Building
```bash
npm run build            # Build frontend for production
npm run tauri build      # Build complete Tauri app bundle
```

### Type Checking & Linting
```bash
npm run check            # Run svelte-check for type errors
npm run check:watch      # Run svelte-check in watch mode
```

### Testing
```bash
# Rust tests
cd src-tauri
cargo test               # Run all Rust tests
cargo test --lib         # Run library tests only
cargo test --test music_library  # Run specific integration test

# Frontend tests
# No test suite currently configured
```

### Preview
```bash
npm run preview          # Preview production build (frontend only)
```

## Project Structure

```
tag-player/
├── src/                          # SvelteKit frontend
│   ├── routes/                   # SvelteKit routes
│   │   ├── +page.svelte         # Main page (grid layout with 4 components)
│   │   ├── +layout.svelte       # Root layout (imports main.css)
│   │   └── +layout.ts           # Layout config (prerender=true, ssr=false)
│   ├── lib/
│   │   ├── components/          # Svelte components
│   │   │   ├── TopBar.svelte    # Folder picker, search, theme toggle
│   │   │   ├── FileList.svelte  # Song list display
│   │   │   ├── TagEditor.svelte # Tag display panel
│   │   │   └── BottomBar.svelte # Playback controls & volume
│   │   └── stores/              # Svelte 5 runes-based stores
│   │       ├── player.svelte.ts # Main state (library, currentSong, playback)
│   │       └── error.svelte.ts  # Error handling
│   ├── app.html                 # HTML template
│   └── main.css                 # Global styles (Tailwind)
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Tauri commands & app setup
│   │   ├── audio.rs             # Audio playback (rodio)
│   │   └── music_library.rs     # File scanning & tag reading (lofty)
│   ├── tests/
│   │   ├── music_library.rs     # Integration tests
│   │   └── music_libraries/     # Test fixtures (empty/, one_file_with_tags/)
│   ├── Cargo.toml
│   ├── tauri.conf.json          # Tauri configuration
│   └── build.rs                 # Build script
├── static/                       # Static assets
├── package.json
├── vite.config.js
├── svelte.config.js
└── tsconfig.json
```

## Code Organization

### Frontend (SvelteKit + Svelte 5)

**Routing**: Uses SvelteKit file-based routing in `src/routes/`
- Root layout at `+layout.svelte` imports global CSS
- Main page at `+page.svelte` contains the 4-panel grid layout
- SSR disabled via `+layout.ts` (Tauri requirement)

**State Management**: Svelte 5 runes (not stores)
- `$state()` for reactive values
- `$derived()` for computed values  
- State classes in `src/lib/stores/*.svelte.ts`
- `playerState` holds library, current song, playback state
- `errorState` derives errors from library.errors

**Component Structure**:
- TopBar: Folder selection (via Tauri dialog), dark mode toggle, search (not yet implemented)
- FileList: Displays songs from library, handles playback on click
- TagEditor: Displays tags for current song in read-only table
- BottomBar: Play/pause, volume control, error display

**Styling**: Tailwind CSS v4 with dark mode support
- Dark mode toggled via `dark` class on `<html>` element
- Preference stored in localStorage
- Gradient backgrounds: `from-purple-700 to-violet-700` (primary accent)
- Color scheme: gray/neutral (light mode), gray-900/violet-900 gradient (dark mode)

**Tauri Integration**:
- Import from `@tauri-apps/api/core` for `invoke()`
- Import from `@tauri-apps/plugin-dialog` for folder picker
- All backend calls use `invoke('command_name', { args })`

### Backend (Rust + Tauri)

**Module Structure**:
- `lib.rs`: Tauri command definitions and app builder
- `audio.rs`: Audio playback thread using rodio
- `music_library.rs`: Music library scanning with walkdir and tag reading with lofty

**Tauri Commands** (defined in `lib.rs`):
```rust
load_and_play(path: String, state: State<AudioPlayer>) -> Result<(), String>
toggle_playback(state: State<AudioPlayer>) -> Result<(), String>
volume_change(volume: f32, state: State<AudioPlayer>) -> Result<(), String>
get_music_library(path: String) -> Library
```

**Audio Architecture**:
- Separate thread for audio playback (spawned in `lib.rs::run()`)
- Commands sent via MPSC channel (`AudioCommand` enum)
- Single `Sink` managed by the audio thread
- Supports: LoadAndPlay, TogglePlayback, VolumeChange

**Music Library Scanning**:
- Uses `walkdir` to recursively traverse directories
- Filters for music files: mp3, wav, flac (case-insensitive)
- Uses `lofty` crate to read audio tags
- Returns `Library { songs: Vec<Song>, errors: Vec<String> }`
- Errors collected for: directory traversal failures, invalid UTF-8 paths, tag reading failures

**Tag Reading** (lofty):
- Reads primary tag from audio file
- Extracts: title, artist, album_artist, album, date, genre, mood, track_number
- Uses `ItemKey::AlbumArtist`, `ItemKey::RecordingDate`, `ItemKey::Mood` for non-standard tags
- Returns `Tags::default()` if no tags found (all `Option::None`)

**Error Handling**:
- `anyhow` crate for error context
- Errors propagated as strings to frontend via `Result<(), String>`
- Library errors collected in `Library.errors` array

## Naming Conventions

### Rust
- **Files**: snake_case (`music_library.rs`, `audio.rs`)
- **Functions**: snake_case (`load_and_play`, `gather_music_library`)
- **Types**: PascalCase (`AudioPlayer`, `Library`, `Song`, `Tags`)
- **Enums**: PascalCase for enum, PascalCase for variants (`AudioCommand::LoadAndPlay`)
- **Constants**: SCREAMING_SNAKE_CASE (none in codebase yet)
- **Module**: Public module `music_library` in `lib.rs`, private module `audio`
- **Crate name**: `tag_player_lib` (note underscore, not hyphen)

### TypeScript/JavaScript
- **Files**: PascalCase for components (`TopBar.svelte`), snake_case for utilities
- **Components**: PascalCase (`TopBar`, `FileList`)
- **Functions**: camelCase (`toggleDarkMode`, `selectFolder`)
- **Variables**: camelCase (`isDarkMode`, `searchQuery`)
- **Interfaces**: PascalCase (`Song`, `Tags`, `Library`)
- **Stores**: camelCase (`playerState`, `errorState`)
- **Routes**: SvelteKit convention (`+page.svelte`, `+layout.ts`)

### Styling
- Tailwind utility classes throughout
- No custom CSS classes beyond Tailwind
- Dark mode classes prefixed with `dark:`

## TypeScript Patterns

### Svelte 5 Runes (State Management)
```typescript
// Define state class
class PlayerState {
  library = $state<Library>({ songs: [], errors: []})
  isPlaying = $derived(false)  // Currently NOT reactive - should be $state
  isLoaded = $state(false)
  currentSong = $state<Song | null>(null)
}

// Export instance
export const playerState = new PlayerState()
```

**Important**: `$derived()` creates computed values, not writable state. The `isPlaying` field should likely be `$state(false)` instead.

### Component State
```typescript
// In components
let volume = $state(50)         // Local reactive state
let isDarkMode = $state(true)   // Local reactive state
```

### Tauri Invocations
```typescript
// Call Rust commands
const result = await invoke('command_name', { argName: value })

// Example with folder picker
const selectedFolder = await open({ directory: true })
playerState.library = await invoke('get_music_library', { path: selectedFolder })

// Example with error handling
try {
  const result = await invoke('load_and_play', { path: song.path })
  if (result !== null) {
    errorState.setError(String(result))
    return
  }
} catch (e) {
  errorState.setError(String(e))
}
```

### Type Definitions
- Rust types serialized to TypeScript via serde
- TypeScript interfaces mirror Rust structs (Tag, Song, Library)
- Optional fields use `Option<T>` in Rust, nullable types in TypeScript

## Rust Patterns

### Tauri Commands
```rust
#[tauri::command]
fn command_name(arg: Type, state: State<SharedState>) -> Result<ReturnType, String> {
    // Implementation
    state.sender.send(Command).map_err(|e| e.to_string())?;
    Ok(())
}
```

### Serde Serialization
```rust
#[derive(serde::Serialize)]
pub struct StructName {
    pub field: Type,
}
```

### Error Handling with anyhow
```rust
use anyhow::{Context, Result};

fn read_tags(path: &Path) -> Result<Tags> {
    let tagged_file = lofty::read_from_path(path)
        .with_context(|| format!("Failed to read audio file: {}", path.display()))?;
    // ...
}
```

### Audio Thread Pattern
```rust
// In lib.rs
let (sender, receiver) = channel();
thread::spawn(move || audio::audio_thread(receiver));

// In audio.rs
pub fn audio_thread(receiver: Receiver<AudioCommand>) {
    loop {
        match receiver.recv() {
            Ok(command) => { /* handle */ },
            Err(_) => break,  // Channel closed
        }
    }
}
```

### File Iteration with Error Collection
```rust
let mut results = Vec::new();
let mut errors = Vec::new();

for entry_result in WalkDir::new(path).into_iter() {
    let entry = match entry_result {
        Ok(e) => e,
        Err(e) => {
            errors.push(format!("Error: {}", e));
            continue;
        }
    };
    // Process entry...
}
```

## Testing Approach

### Rust Tests
- Integration tests in `src-tauri/tests/`
- Test fixtures in `src-tauri/tests/music_libraries/`
- Unit tests in module with `#[cfg(test)]`
- Test naming: `test_function_name_scenario`

**Existing Tests**:
1. `test_gather_music_library_non_existent_folder` - expects 1 error
2. `test_gather_music_library_empty_folder` - expects 0 songs, 0 errors
3. `test_gather_music_library_one_file_with_tags` - verifies tag reading
4. `test_read_tags_nonexistent_file` - unit test for error handling

### Frontend Tests
- No test suite currently configured
- Would typically use Vitest + Testing Library

## Important Gotchas

### 1. SvelteKit SSR/SSG Configuration
- **Must disable SSR** in Tauri apps: `export const ssr = false` in `+layout.ts`
- **Must enable prerendering**: `export const prerender = true`
- Uses `@sveltejs/adapter-static` to generate static files
- Frontend builds to `build/` directory (configured in `tauri.conf.json`)

### 2. Svelte 5 Runes Confusion
- `$derived()` is for computed values (read-only)
- `$state()` is for writable reactive state
- The current `isPlaying = $derived(false)` doesn't make sense - should be `$state(false)`
- Runes must be used in `.svelte.ts` files (not plain `.ts`)

### 3. Tauri Command Return Types
- Commands return `Result<T, String>` where error is serialized to frontend
- `Result<(), String>` for commands with no return value
- Frontend receives `null` on success, error string on failure (check with `if (result !== null)`)

### 4. Audio Thread Communication
- Audio runs in separate thread with MPSC channel
- Commands are fire-and-forget (no response from audio thread)
- Must manage playback state in frontend separately from audio state

### 5. Path Handling
- Rust receives paths as `String` from frontend
- Must convert to `Path` for file operations: `Path::new(&string)`
- Path validation happens implicitly (file operations fail if invalid)
- Test fixtures use relative paths (`./tests/music_libraries/...`)

### 6. Vite Port Configuration
- Vite dev server runs on **port 1420** (hardcoded in `vite.config.js`)
- HMR on port 1421
- `strictPort: true` means dev fails if port unavailable

### 7. Tailwind v4 Setup
- Uses new `@tailwindcss/vite` plugin (not PostCSS)
- Import in `vite.config.js`, not `tailwind.config.js`
- Main CSS file must import Tailwind directives

### 8. Library Name vs Package Name
- Cargo package: `tag-player` (hyphen)
- Cargo library: `tag_player_lib` (underscore + `_lib` suffix)
- Suffix required to avoid name conflicts on Windows

### 9. Lofty Tag Reading
- Some tags require `ItemKey` enum: `AlbumArtist`, `RecordingDate`, `Mood`
- Standard tags have methods: `title()`, `artist()`, `album()`, `genre()`, `track()`
- Returns `Option<&str>` which must be `.to_string()` for owned value
- Always check for primary tag existence: `if let Some(tag) = file.primary_tag()`

### 10. Dark Mode Implementation
- Managed via `dark` class on `<html>` element (Tailwind convention)
- State stored in localStorage, not reactive store
- Must manually add/remove class on document element
- Initialized in `onMount()` to read saved preference

### 11. File Extension Matching
- Extension check is case-insensitive: `.to_lowercase()`
- Uses pattern matching: `matches!(ext, "mp3" | "wav" | "flac")`
- Must check both `.is_file()` and extension (directories can have extensions)

### 12. Error State Architecture
- Rust collects errors during library scan (non-fatal)
- Errors displayed in BottomBar component
- `errorState` derives from `playerState.library.errors`
- ErrorState currently has methods (`setError`, `clearError`) that don't exist - dead code

## Development Workflow

### Starting Development
1. Run `npm install` to install frontend dependencies
2. Run `npm run tauri dev` to start full app in dev mode
3. Tauri will compile Rust backend first, then start Vite dev server

### Making Changes

**Frontend Changes**:
1. Edit Svelte components in `src/`
2. Vite HMR auto-reloads changes
3. Run `npm run check` to verify TypeScript types

**Backend Changes**:
1. Edit Rust files in `src-tauri/src/`
2. Tauri watches for changes and recompiles
3. Run `cargo test` in `src-tauri/` directory to verify tests

**Tauri Commands**:
1. Add command function in `lib.rs` with `#[tauri::command]`
2. Add to `invoke_handler` in `lib.rs::run()`
3. Call from frontend with `invoke('command_name', { args })`
4. TypeScript types must be manually synced (no codegen)

### Adding Dependencies

**Frontend**: 
```bash
npm install <package>
```

**Backend**:
```bash
cd src-tauri
cargo add <crate>
```

### Building for Production
```bash
npm run tauri build
```
This builds both frontend and backend, creates installers in `src-tauri/target/release/bundle/`

## Config Files Reference

### `package.json`
- Scripts: dev, build, preview, check, check:watch, tauri
- Frontend dependencies: Svelte 5, SvelteKit, Tailwind, Tauri API, Lucide icons

### `src-tauri/Cargo.toml`
- Library name: `tag_player_lib` (important for imports in tests)
- Crate types: staticlib, cdylib, rlib
- Dependencies: tauri, rodio, lofty, walkdir, serde, anyhow

### `tauri.conf.json`
- Dev command: `npm run dev`
- Build command: `npm run build`
- Frontend dist: `../build` (relative to src-tauri/)
- Dev URL: `http://localhost:1420`
- Window: 800x600 default size

### `vite.config.js`
- SvelteKit plugin
- Tailwind v4 plugin
- Port: 1420 (strict)
- Ignores `src-tauri/` from watch

### `svelte.config.js`
- Adapter: `@sveltejs/adapter-static` (SSG, not SSR)
- Preprocessor: vitePreprocess

### `tsconfig.json`
- Strict mode enabled
- Module resolution: bundler
- Extends `.svelte-kit/tsconfig.json` (generated)

## TODOs & Known Issues

Based on code inspection:

1. **Search not implemented**: `searchQuery` state exists in TopBar but no filtering logic
2. **Error handling inconsistency**: `errorState` has `setError`/`clearError` methods in comments but they're not implemented (uses `$derived`)
3. **isPlaying bug**: Should be `$state(false)` not `$derived(false)` - currently not reactive
4. **No tag editing**: TagEditor is read-only (display only)
5. **No tests for frontend**: Consider adding Vitest + Testing Library
6. **Volume not persisted**: Reset on app restart
7. **Playback state not synced**: Frontend tracks play/pause separately from audio thread
8. **No playlist support**: Only single-file playback
9. **Limited audio formats**: Only mp3, wav, flac supported
10. **No file watching**: Library doesn't auto-update if files change

## Resources

- [Tauri v2 Docs](https://v2.tauri.app/)
- [SvelteKit Docs](https://kit.svelte.dev/)
- [Svelte 5 Docs](https://svelte-5-preview.vercel.app/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/)
- [rodio Docs](https://docs.rs/rodio/)
- [lofty Docs](https://docs.rs/lofty/)
