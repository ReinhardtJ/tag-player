<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  let musicFile = $state('')
  let isLoaded = $state(false)
  let isPlaying = $state(false)
  let buttonText = $state('Play')
  let error = $state('')
  async function play() {
    try {
      if (!isLoaded) {
        const result = await invoke('load_and_play', { path: musicFile })
        if (result !== null) {
          error = String(result)
          return
        }
        buttonText = 'Pause'
        isLoaded = true
        isPlaying = true
      } else {
        const result = await invoke('toggle_playback')
        error = String(result)
        isPlaying = !isPlaying
        buttonText = isPlaying ? 'Pause' : 'Play'
      }
    } catch (e) {
      error = String(e)
    }

  }
</script>

<main class="container">
  <input placeholder="Enter a music file..." bind:value={musicFile}/>
  <br>
  <button onclick={play}>{ buttonText }</button>
  <p>Error: { error }</p>
</main>

<style>


</style>
