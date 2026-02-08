<div class="h-full gradient-border rounded-3xl overflow-hidden">
  <div class="h-full overflow-auto neo-scrollbar">
    {#if sortedSongs.length > 0}
    <FileListToolbar bind:sortBy bind:sortAscending />
    {/if}
    <div class="p-2">
      {#each sortedSongs as song}
        <div>
          <button
            onclick={() => playerState.play(song)}
            class={`neo-raised-xs bg-neutral-800 hover:bg-neutral-700 rounded-lg p-2 my-2 w-full cursor-pointer text-left flex justify-between items-center
            ${song === playerState.currentSong ? 'bg-linear-to-br dark:from-purple-700 to-violet-700' : ''}`}
          >
            <span class="truncate">{song.name}</span>
            <span class="text-neutral-400 text-sm">{song.tags.size}</span>
          </button>
        </div>
      {/each}
    </div>
  </div>
</div>

<script lang="ts">
  import { playerState, type Song } from '$lib/stores/player.svelte'
  import { orderBy } from 'lodash'
  import FileListToolbar from './FileListToolbar.svelte'

  let sortAscending = $state(true)
  let sortBy: 'name' | 'tags' = $state('name')

  const sortedSongs = $derived.by(() => {
    const songs = [...playerState.filteredSongs]
    const sortOrder = sortAscending ? 'asc' : 'desc'
    if (sortBy === 'name') {
      return orderBy<Song>(songs, ['name'], [sortOrder])
    } else {
      return orderBy<Song>(songs, [song => song.tags.size], [sortOrder])
    }
  })
</script>

<style>
</style>
