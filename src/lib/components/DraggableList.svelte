<!-- 
  Generic draggable list component that allows reordering items via drag and drop.
  Shows a visual indicator (purple line) where the dragged item will be inserted.
-->
<div class="flex flex-col gap-1" bind:this={containerElement}>
  {#each items as item, index (getKey(item, index))}
    <!-- Insertion indicator above each item -->
    {#if dragOverIndex === index && draggedIndex !== null && draggedIndex !== index}
      <div class="h-1 bg-purple-500 rounded-full shadow-lg shadow-purple-500/50 -my-0.5 transition-opacity duration-150 opacity-0 animate-fade-in"></div>
    {/if}
    <div
      data-drag-index={index}
      class="transition-all {draggedIndex === index ? 'opacity-30' : ''}"
    >
      <div class="flex items-center gap-2">
        <!-- Drag handle icon -->
        <div
          role="button"
          tabindex="0"
          onmousedown={(e) => handleMouseDown(e, index)}
          class="cursor-grab active:cursor-grabbing text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 select-none"
        >
          <GripVertical size={16} />
        </div>
        <div class="flex-1">
          {@render children(item, index)}
        </div>
      </div>
    </div>
  {/each}
  <!-- Additional insertion indicator after last item -->
  {#if dragOverIndex === items.length && draggedIndex !== null}
    <div class="h-1 bg-purple-500 rounded-full shadow-lg shadow-purple-500/50 -my-0.5 transition-opacity duration-150 opacity-0 animate-fade-in"></div>
  {/if}
</div>

<script lang="ts" generics="T">
  import { GripVertical } from '@lucide/svelte'

  interface Props {
    items: T[]
    onReorder: (newItems: T[]) => void
    keyFn?: (item: T, index: number) => string | number
    children: (item: T, index: number) => any
  }

  let { items, onReorder, keyFn, children }: Props = $props()

  // Track which item is being dragged and where it's hovering
  let draggedIndex = $state<number | null>(null)
  let dragOverIndex = $state<number | null>(null)
  let containerElement: HTMLDivElement

  // Start dragging: store the dragged item's index and attach mouse listeners
  function handleMouseDown(e: MouseEvent, index: number) {
    e.preventDefault()
    draggedIndex = index

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  }

  // While dragging: determine where the item should be inserted based on mouse position
  function handleMouseMove(e: MouseEvent) {
    if (draggedIndex === null || !containerElement) {
      return
    }

    const rows = containerElement.querySelectorAll('[data-drag-index]')
    let foundIndex: number | null = null

    // Find which row the mouse is over by comparing to each row's midpoint
    for (let i = 0; i < rows.length; i++) {
      const row = rows[i] as HTMLElement
      const rect = row.getBoundingClientRect()
      const midpoint = rect.top + rect.height / 2

      // If mouse is above the midpoint, insert before this row
      if (e.clientY < midpoint) {
        foundIndex = parseInt(row.getAttribute('data-drag-index') || '0', 10)
        break
      }
    }

    // If mouse is below all rows, insert at the end
    if (foundIndex === null && rows.length > 0) {
      const lastRow = rows[rows.length - 1] as HTMLElement
      foundIndex = parseInt(lastRow.getAttribute('data-drag-index') || '0', 10) + 1
    }

    dragOverIndex = foundIndex
  }

  // Finish dragging: reorder the items and clean up
  function handleMouseUp() {
    window.removeEventListener('mousemove', handleMouseMove)
    window.removeEventListener('mouseup', handleMouseUp)

    if (draggedIndex !== null && dragOverIndex !== null && draggedIndex !== dragOverIndex) {
      const draggedItem = items[draggedIndex]
      const newItems = [...items]
      newItems.splice(draggedIndex, 1)

      // Adjust insert position: if dragging downward, account for the removed item
      const insertIndex = dragOverIndex > draggedIndex ? dragOverIndex - 1 : dragOverIndex
      newItems.splice(insertIndex, 0, draggedItem)

      onReorder(newItems)
    }

    draggedIndex = null
    dragOverIndex = null
  }

  const getKey = (item: T, index: number) => {
    return keyFn ? keyFn(item, index) : index
  }
</script>

<style>
  .animate-fade-in {
    animation: fadeIn 150ms ease-in forwards;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>