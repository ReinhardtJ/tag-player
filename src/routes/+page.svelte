<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'
  import { open } from '@tauri-apps/plugin-dialog'

  let isLoaded = $state(false)
  let isPlaying = $state(false)
  let buttonText = $state('Play')
  let error = $state('')
  let songList = $state([] as MusicFile[])
  let volume = $state(50)

  interface MusicFile {
    path: string
    name: string
  }


  async function onVolumeChange() {
    await invoke('volume_change', { volume: volume / 100 })
  }

  async function play(song: MusicFile) {
    try {
      const result = await invoke('load_and_play', { path: song.path })
      if (result !== null) {
        error = String(result)
        return
      }
      buttonText = 'Pause'
      isLoaded = true
      isPlaying = true
    } catch (e) {
      error = String(e)
    }

  }

  async function togglePlayback() {
    const result = await invoke('toggle_playback')
    if (result !== null) {
      error = String(result)
      return
    }
    isPlaying = !isPlaying
    buttonText = isPlaying ? 'Pause' : 'Play'
  }

  async function selectFolder() {
    const selectedFolder = await open({ directory: true })
    songList = await invoke('get_music_library', { path: selectedFolder })
  }

</script>

<main>
  <div class="grid grid-cols-2 grid-rows-[auto_1fr_auto] h-screen gap-2">
    <div class="col-span-2">
      <button onclick={selectFolder}>Select Music Folder</button>
    </div>
    <div class="h-full overflow-auto ml-2 my-2 p-2 bg-gray-300 rounded-lg">
      {#each songList as song}
        <div>
          <button
              onclick={() => play(song)}
              class="bg-purple-400 rounded-lg p-1 my-1 w-full cursor-pointer"
          >
            {song.name}
          </button>
        </div>
      {/each}
    </div>
    <div class="h-full bg-gray-300 rounded-lg my-2 p-2 mr-2">
      Taaaags
    </div>
    <div class="col-span-2 p-2 bg-gray-300 rounded-lg m-2 ">
      <input type="range" min="0" max="100" bind:value={volume} oninput={onVolumeChange}>
      <button
          onclick={togglePlayback}
          class="bg-purple-400 rounded-2xl p-1 m-1"
      >
        { buttonText }
      </button>
      <p>Error: { error }</p>
    </div>
  </div>
</main>

<style>


</style>
