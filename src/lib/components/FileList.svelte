<div class="h-full gradient-border rounded-3xl overflow-hidden">
  <div class="h-full overflow-auto p-2 neo-scrollbar">
    {#each playerState.filteredSongs as song}
      <div>
        <button
          onclick={() => playerState.play(song)}
          class={`neo-raised-xs bg-neutral-800 hover:bg-neutral-700 rounded-lg p-2 my-1.5 w-full cursor-pointer text-left
          ${song === playerState.currentSong ? 'bg-linear-to-br dark:from-purple-700 to-violet-700' : ''}`}
        >
          {song.name}
        </button>
      </div>
    {/each}
  </div>
</div>

<script lang="ts">
  import { playerState } from '$lib/stores/player.svelte'
</script>

<style>

</style>
