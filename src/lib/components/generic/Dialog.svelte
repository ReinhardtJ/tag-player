{#if dialogStore.open}
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="z-50 fixed inset-0 flex items-center justify-center" onclick={() => dialogStore.close()}>
    <div class="bg-neutral-800 neo-raised-lg p-4" onclick={(e) => e.stopPropagation()}>
        <div class="text-white">
            <h2 class="text-lg font-bold mb-2">MusicBrainz Search</h2>
            <p>Search results will appear here...</p>
        </div>
    </div>
</div>
{/if}

<script lang="ts">
    import { useDialogStore } from '$lib/stores/dialogStore.svelte'

    const dialogStore = useDialogStore()
</script>