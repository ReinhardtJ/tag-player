<div class="sticky top-2 flex justify-center m-2">
  <div class="px-4 py-2 bg-neutral-800 rounded-3xl flex items-center gap-2 neo-raised-sm w-fit">
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
      class="bg-linear-to-br from-purple-700 to-violet-700/75 text-white rounded-lg px-2 py-1.5 w-8 h-8 hover:bg-neutral-700 focus:outline-none"
    >
      {#if sortAscending}
        <ArrowUp size={12} />
      {:else}
        <ArrowDown size={12} />
      {/if}
    </button>
  </div>
</div>

<script lang="ts">
  import { ArrowUp, ArrowDown } from '@lucide/svelte'

  interface Props {
    sortBy: string
    sortAscending: boolean
    sortOptions: string[]
  }

  function toLabel(option: string) {
    return option.charAt(0).toUpperCase() + option.slice(1).toLowerCase()
  }

  let { sortBy = $bindable(), sortAscending = $bindable(), sortOptions }: Props = $props()
</script>
