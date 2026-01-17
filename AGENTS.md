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
npm run format           # Format code with Prettier
```

### Testing
```bash
# Rust tests
cd src-tauri
cargo test               # Run all Rust tests
cargo test --lib         # Run library tests only
cargo test --test music_library  # Run specific integration test
cargo test test_function_name  # Run single test by name

# Frontend tests
npm run test            # Run Vitest tests
npm run test:ui         # Run Vitest with UI
```

### Preview
```bash
npm run preview          # Preview production build (frontend only)
```

## Code Style Guidelines

### TypeScript/JavaScript

**Formatting**: Prettier with `.prettierrc` configuration:
- 2 spaces for indentation
- Single quotes
- No semicolons
- 100 character line width
- Trailing commas: none

**Naming Conventions**:
- Components: PascalCase (`TopBar.svelte`, `FileList.svelte`)
- Functions: camelCase (`toggleDarkMode`, `selectFolder`)
- Variables: camelCase (`isDarkMode`, `searchQuery`)
- Interfaces: PascalCase (`Song`, `Tags`, `Library`)
- Stores: camelCase (`playerState`, `errorState`)
- Files: snake_case for utilities, PascalCase for components

**Imports**:
- Group imports by source (framework, local, third-party)
- Use named imports over default imports when possible
- Sort imports alphabetically within groups

**Types**:
- Use TypeScript interfaces for data structures
- Nullable fields use `Type | null`
- Svelte 5 runes: `$state()` for writable state, `$derived()` for computed values

**Error Handling**:
- Use try/catch blocks for Tauri invocations
- Check for null results from Rust commands
- Propagate errors to errorState for display

### Rust

**Formatting**: Standard Rustfmt (no custom configuration)
- 4 spaces for indentation
- Snake case for functions and variables
- PascalCase for types and enums
- SCREAMING_SNAKE_CASE for constants

**Naming Conventions**:
- Files: snake_case (`music_library.rs`, `audio.rs`)
- Functions: snake_case (`load_and_play`, `gather_music_library`)
- Types: PascalCase (`AudioPlayer`, `Library`, `Song`, `Tags`)
- Enums: PascalCase for enum and variants (`AudioCommand::LoadAndPlay`)
- Modules: snake_case, public modules in `lib.rs`

**Imports**:
- Group imports by source (std, external crates, local modules)
- Use `use` statements at top of file
- Sort imports alphabetically within groups

**Error Handling**:
- Use `anyhow` crate for error context
- Propagate errors with `?` operator
- Convert errors to strings for frontend: `.map_err(|e| e.to_string())?`
- Collect non-fatal errors in `Vec<String>` for library operations

**Patterns**:
- Tauri commands: `#[tauri::command]` attribute
- Serde serialization: `#[derive(serde::Serialize)]`
- Audio thread: MPSC channel communication
- File operations: `walkdir` for directory traversal
- Tag reading: `lofty` crate with error handling

### Svelte

**Component Structure**:
- Use Svelte 5 runes for state management
- Keep components focused and small
- Use `$:` reactive statements for derived values
- Place logic in separate `.svelte.ts` files when complex

**Styling**:
- Tailwind CSS v4 utility classes
- No custom CSS classes
- Dark mode: `dark:` prefix for dark mode variants
- Color scheme: purple/violet gradients

**File Organization**:
- Components in `src/lib/components/`
- Stores in `src/lib/stores/`
- Routes in `src/routes/`
- Static assets in `static/`

## Testing Conventions

### Rust Tests
- Integration tests in `src-tauri/tests/`
- Unit tests in module with `#[cfg(test)]`
- Test fixtures in `src-tauri/tests/music_libraries/`
- Test naming: `test_function_name_scenario`
- Use `cargo test test_name` to run single test

### Frontend Tests
- Vitest configuration in `vitest.config.ts`
- Component tests with `.test.ts` extension
- Use `@testing-library/svelte` patterns
- Run with `npm run test`

## Build & Lint Process

1. **Type Checking**: `npm run check` (svelte-check)
2. **Formatting**: `npm run format` (Prettier)
3. **Rust Tests**: `cargo test` in src-tauri/
4. **Frontend Tests**: `npm run test`
5. **Build**: `npm run tauri build`

## Important Gotchas

1. **Svelte 5 Runes**: `$state()` for writable, `$derived()` for computed
2. **Tauri Commands**: Return `Result<T, String>` for frontend compatibility
3. **Path Handling**: Convert String to Path in Rust: `Path::new(&string)`
4. **Audio Thread**: Fire-and-forget commands, no response from thread
5. **Error State**: Collect errors in `Library.errors` array
6. **Dark Mode**: Managed via `dark` class on HTML element
7. **File Extensions**: Case-insensitive matching for audio files

## Development Workflow

1. **Frontend Changes**: Edit in `src/`, Vite HMR auto-reloads
2. **Backend Changes**: Edit in `src-tauri/src/`, Tauri recompiles
3. **Testing**: Run relevant test suites before committing
4. **Formatting**: Run `npm run format` before commits
5. **Type Checking**: Run `npm run check` to verify types

## Resources

- [Tauri v2 Docs](https://v2.tauri.app/)
- [SvelteKit Docs](https://kit.svelte.dev/)
- [Svelte 5 Docs](https://svelte-5-preview.vercel.app/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/)
- [rodio Docs](https://docs.rs/rodio/)
- [lofty Docs](https://docs.rs/lofty/)
