{#if dialogStore.open}
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="z-50 fixed inset-0 flex items-center justify-center bg-black/50" onclick={() => dialogStore.close()}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="w-[42rem] max-w-[90vw] max-h-[80vh] flex flex-col gap-3 bg-neutral-800 neo-raised-lg rounded-2xl p-4"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="flex items-center justify-between">
      <h2 class="text-lg font-bold">MusicBrainz Search</h2>
      <button onclick={() => dialogStore.close()} class="btn-secondary">
        <X size={16} />
      </button>
    </div>

    {#if musicBrainzStore.isSearching}
      <div class="flex items-center justify-center gap-2 py-10 text-neutral-400">
        <LoaderCircle class="animate-spin" size={16} />
        <span>Searching MusicBrainz…</span>
      </div>
    {:else if musicBrainzStore.searchError}
      <p class="py-6 text-center text-red-500">{musicBrainzStore.searchError}</p>
    {:else if musicBrainzStore.results.length === 0}
      <p class="py-6 text-center text-neutral-400">No matches found.</p>
    {:else}
      <div class="flex flex-col gap-2 overflow-auto neo-scrollbar">
        {#each musicBrainzStore.results as recording (recording.id)}
          {@const release = formatRelease(recording)}
          <button
            onclick={() => applyRecording(recording)}
            class="w-full rounded-lg bg-neutral-800 p-2 text-left neo-raised-xs hover:bg-neutral-700 hover:cursor-pointer"
          >
            <div class="flex items-baseline justify-between gap-3">
              <span class="truncate font-semibold">{recording.title}</span>
              <span class="shrink-0 text-sm text-neutral-400">{formatArtistCredit(recording)}</span>
            </div>
            {#if release}
              <div class="truncate text-sm text-neutral-400">{release}</div>
            {/if}
            {#if recording.disambiguation}
              <div class="truncate text-xs text-neutral-500">{recording.disambiguation}</div>
            {/if}
          </button>
        {/each}
      </div>
    {/if}

    {#if musicBrainzStore.applyError}
      <p class="text-center text-sm text-red-500">{musicBrainzStore.applyError}</p>
    {/if}
  </div>
</div>
{/if}

<script lang="ts">
  import { X, LoaderCircle } from '@lucide/svelte'
  import { useDialogStore } from '$lib/stores/dialogStore.svelte'
  import { useMusicBrainzStore } from '$lib/stores/musicBrainzStore.svelte.ts'
  import {
    formatArtistCredit,
    formatRelease,
    type MusicBrainzRecording
  } from '$lib/stores/musicBrainzTypes.ts'

  const dialogStore = useDialogStore()
  const musicBrainzStore = useMusicBrainzStore()

  async function applyRecording(recording: MusicBrainzRecording) {
    const applied = await musicBrainzStore.applyRecording(recording)
    if (applied) 
      dialogStore.close()
  }
</script>
