# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A desktop music player built with Tauri (Rust backend) + SvelteKit (frontend). The application features a custom audio engine using Symphonia for decoding and cpal for output, enabling precise seeking and position tracking.

## Build & Development Commands

### Development
```bash
npm run dev          # Start SvelteKit dev server + Tauri dev mode
npm run tauri dev    # Start Tauri dev mode directly
```

### Build
```bash
npm run build        # Build SvelteKit frontend for production
npm run tauri build  # Build complete Tauri application (frontend + Rust)
```

### Type Checking & Linting
```bash
npm run check              # Run svelte-check for type errors
npm run check:watch        # Run svelte-check in watch mode
```

### Rust Development
```bash
cd src-tauri
cargo check               # Quick compile check
cargo build               # Build debug binary
cargo build --release     # Build optimized release binary
cargo test                # Run Rust tests (if any)
```

## Architecture Overview

### Multi-Threaded Audio Engine

The application uses a custom 4-thread audio architecture (detailed in `SYMPHONIA_ARCHITECTURE.md`):

1. **Tauri Thread** - Receives frontend invoke() calls and sends commands via mpsc channel
2. **Audio Thread** (src-tauri/src/audio/audio_thread.rs) - Main coordinator that:
   - Manages lifecycle and owns the cpal stream
   - Spawns/stops decoder threads
   - Forwards commands between frontend and decoder
3. **Decoder Thread** (src-tauri/src/audio/decoding.rs) - Per-song thread that:
   - Uses Symphonia to decode audio packets
   - Converts samples to f32
   - Pushes to lock-free ring buffer
   - Handles seeking via format_reader.seek()
4. **Position Updater Thread** (src-tauri/src/audio/position_updater.rs) - Emits position events to frontend every 200ms

### Key Data Flow Patterns

**Command Flow**: Frontend → Tauri Commands → mpsc::channel → Audio Thread → Decoder Thread

**Audio Flow**: Decoder Thread → ringbuf (lock-free) → cpal Callback → OS Audio Output

**Event Flow**: Position Updater → app_handle.emit() → Frontend Listeners

**Shared State**: `Arc<Mutex<PlaybackState>>` accessed by all threads for:
- is_playing, is_paused, volume
- current_position_samples, sample_rate, duration_samples

### Critical Implementation Details

**Ring Buffer**:
- Size: 48000 * 2 * 4 = 384,000 samples (4 seconds of stereo audio at 48kHz)
- Lock-free (ringbuf crate) for real-time audio safety
- Decoder blocks when buffer is full (backpressure)

**Audio Callback** (src-tauri/src/audio/cpal_stream.rs):
- Runs on OS audio thread - must be fast!
- Reads from ring buffer
- Applies volume
- Outputs silence when paused
- Updates current_position_samples

**Seeking**:
- Frontend → AudioCommand::Seek(seconds) → Audio Thread converts to samples → DecoderCommand::Seek(samples)
- Decoder calls format_reader.seek() with calculated timestamp
- Decoder calls decoder.reset() to clear decoder state
- Updates state.current_position_samples

### Module Organization

**Rust Backend** (src-tauri/src/):
- `lib.rs` - Tauri command definitions, setup with mpsc channel
- `music_library.rs` - Walks directories, extracts tags with lofty
- `audio/mod.rs` - Module exports
- `audio/shared.rs` - Shared types (AudioCommand, PlaybackState, AudioPlayer)
- `audio/audio_thread.rs` - Main coordinator loop
- `audio/decoding.rs` - Symphonia decoder thread
- `audio/cpal_stream.rs` - Audio output stream creation
- `audio/position_updater.rs` - Position event emitter

**Frontend** (src/):
- `routes/+page.svelte` - Main page
- `routes/+layout.svelte` - App layout
- `lib/components/` - Reusable Svelte components
  - `FileList.svelte` - Music library display
  - `PlayButton.svelte` - Play/pause control
  - `VolumeSlider.svelte` - Volume control
  - `BottomBar.svelte` - Playback controls
  - `TopBar.svelte` - Navigation
  - `OpenFolderButton.svelte` - Folder picker
  - `SearchBar.svelte` - Library search
  - `TagEditor.svelte` - Metadata editing
  - `ThemeToggle.svelte` - Dark/light mode

## Important Development Patterns

### Rust Thread Safety
- **Never hold mutex locks during I/O or sleep** - causes audio glitches
- **Lock pattern**: Lock → Read/Write → Drop immediately
- All shared state is `Arc<Mutex<T>>` except ring buffer (lock-free)

### Audio Callback Constraints
- Must be non-blocking and fast (<1ms)
- No allocations, file I/O, or heavy computation
- Only reads from ring buffer and shared state

### Adding New Audio Commands
1. Add variant to `AudioCommand` enum in `audio/shared.rs`
2. Add Tauri command in `lib.rs` that sends the command
3. Handle in `audio_thread.rs` match statement
4. Add to `invoke_handler` macro in `lib.rs`

### Frontend-Backend Communication
**Invoke (Frontend → Backend)**:
```typescript
import { invoke } from '@tauri-apps/api/core';
await invoke('load_and_play', { path: '/path/to/song.mp3' });
```

**Events (Backend → Frontend)**:
```typescript
import { listen } from '@tauri-apps/api/event';
listen('playback:position', (event) => {
  const { position, duration } = event.payload;
});
```

Emitted events:
- `playback:position` - { position: f64, duration: f64 } every 200ms
- `playback:duration` - f64 when song loads

## Technology Stack

**Backend**:
- Tauri 2 - Desktop app framework
- Symphonia - Audio decoding (mp3, aac, flac, wav, vorbis, ogg)
- cpal - Cross-platform audio output
- ringbuf - Lock-free ring buffer
- lofty - Audio metadata extraction
- anyhow - Error handling

**Frontend**:
- SvelteKit 2 with adapter-static (SSG for Tauri)
- Svelte 5 - UI framework
- TailwindCSS 4 - Styling
- TypeScript - Type safety
- Vite - Build tool

## Known Limitations & Future Work

1. **Buffer Management**: When loading a new song, old ring buffer data may play briefly (~100-500ms). Solution: Recreate cpal stream on LoadAndPlay.

2. **Position Accuracy**: Position tracking counts consumed samples. May drift slightly over long periods. Solution: Sync with Symphonia's frame position.

3. **No Gapless Playback**: Silence between tracks in a queue. Solution: Append next track's decoder to same buffer.

4. **Seek Buffer Artifacts**: Ring buffer may contain old audio after seek. Solution: Drain or recreate buffer on seek.

## Common Tasks

### Adding Support for New Audio Format
1. Add feature to symphonia dependency in `src-tauri/Cargo.toml`
2. Test with sample file - decoder should auto-detect format

### Debugging Audio Issues
- Enable verbose logging in audio modules (search for println! statements)
- Check ring buffer health: producer.remaining() and consumer.len()
- Monitor position drift between calculated and expected position
- Verify mutex locks are held for microseconds, not milliseconds

### Modifying Frontend UI
- Components use TailwindCSS utilities
- Neumorphic design pattern (soft shadows, depth)
- Dark mode via CSS variables
- Tauri invoke calls are async, handle errors appropriately
