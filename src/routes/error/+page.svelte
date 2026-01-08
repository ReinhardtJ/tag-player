<script lang="ts">
  import { ChevronLeft } from '@lucide/svelte'
  import { errorState } from '$lib/stores/error.svelte'
</script>

<main class="h-screen p-2">
  <a
    href="/"
    class="neo-raised-sm p-2 rounded-lg bg-gray-300 dark:bg-neutral-700 hover:dark:bg-neutral-600 inline-flex items-center justify-center mb-4"
  >
    <ChevronLeft size={20} />
  </a>
  <h1 class="text-xl">Errors:</h1>
  <div class="text-red-400">
    {errorState.errors}
  </div>
</main>
