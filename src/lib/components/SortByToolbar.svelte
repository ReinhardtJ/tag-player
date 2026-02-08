<div class="flex items-center gap-2">
  <span>Sort by</span>
  <select
    bind:value={sortBy}
    class="bg-linear-to-br from-purple-700 to-violet-700/75 text-white rounded-lg px-3 py-1.5 h-8 focus:outline-none"
  >
    {#each sortOptions as option}
      <option value={option}>{toLabel(option)}</option>
    {/each}
  </select>

  <button
    onclick={() => (sortAscending = !sortAscending)}
    class="bg-linear-to-br from-purple-700 to-violet-700/75 text-white rounded-lg px-1 py-1.5 w-8 h-8 hover:bg-neutral-700 focus:outline-none flex items-center justify-center"
  >
    {#if sortAscending}
      <ArrowUp size={16} />
    {:else}
      <ArrowDown size={16} />
    {/if}
  </button>
</div>

<script lang="ts">
  import { ArrowUp, ArrowDown } from '@lucide/svelte'

  function toLabel(option: string) {
    return option.charAt(0).toUpperCase() + option.slice(1).toLowerCase()
  }

  let {
    sortBy = $bindable(),
    sortAscending = $bindable(),
    sortOptions
  }: {
    sortBy: string
    sortAscending: boolean
    sortOptions: string[]
  } = $props()
</script>
