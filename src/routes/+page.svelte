<main>
  <div class="grid grid-cols-2 grid-rows-[auto_1fr_auto] h-screen gap-4">
    <div class="col-span-2 mt-2 mx-4">
      <TopBar />
    </div>
    <div class="ml-2 h-full overflow-hidden">
      <FileList />
    </div>
    <div class="mr-2 h-full overflow-hidden">
      <TagEditor />
    </div>
    <div class="col-span-2 mx-2 mb-2">
      <BottomBar />
    </div>
  </div>
</main>

<script lang="ts">
  import TopBar from '$lib/components/topBar/TopBar.svelte'
  import FileList from '$lib/components/fileList/FileList.svelte'
  import TagEditor from '$lib/components/TagEditor.svelte'
  import BottomBar from '$lib/components/bottomBar/BottomBar.svelte'
</script>
