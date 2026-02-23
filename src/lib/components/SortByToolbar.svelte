<div class="shrink-0 flex items-center gap-2">
  <span>Sort by</span>
  <select
    bind:value={sortBy}
    class="btn-primary h-8 focus:outline-none"
  >
    {#each sortOptions as option (option)}
      <option value={option}>{toLabel(option)}</option>
    {/each}
  </select>

  <button
    onclick={toggleSortOrder}
    class="btn-primary h-8 focus:outline-none flex items-center justify-center"
  >
    {#if sortOrder === 'asc'}
      <ArrowUp size={16} />
    {:else}
      <ArrowDown size={16} />
    {/if}
  </button>
</div>

<script lang="ts">
  import { ArrowUp, ArrowDown } from '@lucide/svelte'
  import type { SortOrder } from '$lib/components/SortByToolbar.types.ts'

  function toLabel(option: string) {
    return option.charAt(0).toUpperCase() + option.slice(1).toLowerCase()
  }

  function toggleSortOrder() {
    if (sortOrder === 'asc')
      sortOrder = 'desc'
    else
      sortOrder = 'asc'
  }

  let {
    sortBy = $bindable(),
    sortOrder = $bindable(),
    sortOptions
  }: {
    sortBy: string
    sortOrder?: SortOrder | undefined
    sortOptions: string[]
  } = $props()
</script>
