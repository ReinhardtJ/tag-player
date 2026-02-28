<div class="h-full gradient-border rounded-3xl overflow-hidden">
  <div class="h-full overflow-auto neo-scrollbar">
    {#if playerStore.library.songs.length > 0}
      <div
        class="sticky flex items-center top-2 m-2 px-2 py-2 bg-neutral-800 rounded-2xl neo-raised-sm gap-2"
      >
        <SortByToolbar bind:sortBy bind:sortOrder sortOptions={['name', 'tags']} />
        <SearchBar></SearchBar>
      </div>
    {/if}
    <div class="p-2">
      {#each sortedSongs as song (song.path)}
        <div>
          <button
            onclick={() => playerStore.play(song)}
            class={`neo-raised-xs bg-neutral-800 hover:bg-neutral-700 rounded-lg p-2 my-2 w-full cursor-pointer text-left flex justify-between items-center gap-3
            ${song === playerStore.currentSong ? 'bg-linear-to-br dark:from-purple-700 to-violet-700' : ''}`}
          >
            <span class="w-14 h-14 rounded bg-neutral-700 flex items-center justify-center" >
              {#if song.cover_base64}
                <img src={song.cover_base64} alt="Cover" class="w-full h-full object-cover" />
              {:else}
                <Music class="w-6 h-6 text-neutral-400" />
              {/if}
            </span>
            <span class="truncate flex-1">{song.name}</span>
            <span class="text-neutral-400 text-sm">{song.tags.size}</span>
          </button>
        </div>
      {/each}
    </div>
  </div>
</div>

<script lang="ts">
  import { orderBy } from 'lodash'
  import SortByToolbar from '../SortByToolbar.svelte'
  import { usePlayerStore } from '$lib/stores/playerStore.svelte'
  import SearchBar from '../topBar/SearchBar.svelte'
  import type { Song } from '$lib/stores/playerTypes.ts'
  import type { SortOrder } from '$lib/components/SortByToolbar.types.ts'
  import { Music } from '@lucide/svelte'

  const playerStore = usePlayerStore()

  let sortOrder = $state<SortOrder>('asc')
  let sortBy: 'name' | 'tags' = $state('name')

  const sortedSongs = $derived.by(() => {
    const songs = [...playerStore.filteredSongs]
    if (sortBy === 'name') {
      return orderBy<Song>(songs, ['name'], [sortOrder])
    } else {
      return orderBy<Song>(songs, [(song) => song.tags.size], [sortOrder])
    }
  })
</script>

<style>
</style>
