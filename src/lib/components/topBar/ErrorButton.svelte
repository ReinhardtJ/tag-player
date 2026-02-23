{#if errorStore.errors.length > 0}
  <a href={resolve("/error")} class="btn-secondary">Errors ({errorStore.errors.length})</a>
{/if}

<script lang="ts">
  import { useErrorStore } from '$lib/stores/errorStore.svelte'
  import { resolve } from '$app/paths'

  const errorStore = useErrorStore()
</script>
