<div class="p-2 rounded-lg">
  <div class="pb-4">
    <TrackPositionSlider />
  </div>
  <div class="flex items-center justify-between">
    <!-- left: spacing -->
    <div class="flex-1"></div>

    <!-- center: Play/Pause button -->
    <div class="flex-1 flex justify-center">
      <PlayButton />
    </div>

    <!-- right: Volume controls -->
    <div class="flex-1 flex justify-end">
      <VolumeSlider />
    </div>
  </div>
</div>

<script lang="ts">
  import { errorState } from '$lib/stores/error.svelte'
  import PlayButton from './PlayButton.svelte'
  import VolumeSlider from './VolumeSlider.svelte'
  import TrackPositionSlider from './TrackPositionSlider.svelte'
</script>
