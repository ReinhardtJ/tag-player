<script lang="ts">
  import TopBar from '$lib/components/TopBar.svelte'
  import FileList from '$lib/components/FileList.svelte'
  import TagEditor from '$lib/components/TagEditor.svelte'
  import BottomBar from '$lib/components/BottomBar.svelte'
</script>

<main class="bg-white dark:bg-linear-to-br dark:from-gray-900 dark:to-violet-900">
  <div class="grid grid-cols-2 grid-rows-[auto_1fr_auto] h-screen gap-1.5">
    <div class="col-span-2 mt-1.5 mx-1.5">
      <TopBar />
    </div>
    <div class="ml-1.5 h-full overflow-hidden">
      <FileList />
    </div>
    <div class="mr-1.5 h-full overflow-hidden">
      <TagEditor />
    </div>
    <div class="col-span-2 mx-2 mb-1.5">
      <BottomBar />
    </div>
  </div>
</main>
