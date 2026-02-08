<div class="flex items-center gap-2">
  <Volume2 size={20} />
  <input
    type="range"
    min="0"
    max="100"
    bind:value={volume}
    oninput={onVolumeChange}
    class="w-24 neo-slider"
    style="--volume-percent: {volume}%"
  />
</div>

<script lang="ts">
  import { Volume2 } from '@lucide/svelte'
  import { playerState } from '$lib/stores/player.svelte'

  let volume = $state(50)

  async function onVolumeChange() {
    await playerState.changeVolume(volume / 100)
  }
</script>

<style>
  /* Remove default styling */
  .neo-slider {
    -webkit-appearance: none;
    appearance: none;
    background: transparent;
    cursor: pointer;
  }

  /* Track - Neomorphic inset with progressive purple fill and rounded end */
  .neo-slider::-webkit-slider-runnable-track {
    width: 100%;
    height: 18px;
    background:
      radial-gradient(
        circle at var(--volume-percent) 50%,
        var(--color-violet-700) 0%,
        var(--color-violet-700) 9px,
        transparent 9px
      ),
      linear-gradient(
        to right,
        var(--color-purple-700) 0%,
        var(--color-violet-700) var(--volume-percent),
        var(--color-neutral-800) var(--volume-percent),
        var(--color-neutral-800) 100%
      );
    border-radius: 12px;
    box-shadow:
      inset 2px 2px 4px rgba(0, 0, 0, 0.4),
      inset -2px -2px 4px rgba(255, 255, 255, 0.05);
  }

  /* Thumb - Invisible but still draggable */
  .neo-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 0;
    height: 18px;
    margin-top: 0;
  }
</style>
