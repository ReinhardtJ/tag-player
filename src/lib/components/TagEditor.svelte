<script>
  import { playerState } from '$lib/stores/player.svelte'

  const tags = $derived(playerState.currentSong?.tags)
</script>

<div class="h-full gradient-border rounded-3xl p-2">
  {#if tags !== undefined }
    <table class="w-full">
      <tbody>
        <tr>
          <td class="font-semibold pr-4">Title</td>
          <td>{tags.title ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Artist</td>
          <td>{tags.artist ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Album Artist</td>
          <td>{tags.album_artist ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Album</td>
          <td>{tags.album ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Date</td>
          <td>{tags.date ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Genre</td>
          <td>{tags.genre ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Mood</td>
          <td>{tags.mood ?? '-'}</td>
        </tr>
        <tr>
          <td class="font-semibold pr-4">Track Number</td>
          <td>{tags.track_number ?? '-'}</td>
        </tr>
      </tbody>
    </table>
  {/if}
</div>
