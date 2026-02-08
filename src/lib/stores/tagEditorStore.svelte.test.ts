import { describe, it, expect } from 'vitest'
import { sortTagFieldsByRelevance, type TagField } from './tagEditorStore.svelte.js'

describe('sortTagFieldsByRelevance', () => {
  it('descending: places priority tags before non-priority tags when sorting', () => {
    const fields: TagField[] = [
      { id: '1', tagName: 'Prio2', tagValue: 'value1' },
      { id: '2', tagName: 'Prio1', tagValue: 'value2' }
    ]

    const result = sortTagFieldsByRelevance(fields, ['Prio1', 'Prio2'], false)

    expect(result.map((f) => f.tagName)).toEqual(['Prio1', 'Prio2'])
  })
  
  it('ascending: places non-priority before priority tags', () => {
    const fields: TagField[] = [
      { id: '1', tagName: 'Prio2', tagValue: 'value1' },
      { id: '2', tagName: 'Prio1', tagValue: 'value2' }
    ]

    const result = sortTagFieldsByRelevance(fields, ['Prio1', 'Prio2'], true)

    expect(result.map((f) => f.tagName)).toEqual(['Prio2', 'Prio1'])
  })

  it('descending: places priority tags before non-priority tags', () => {
    const fields: TagField[] = [
      { id: '1', tagName: 'Other1', tagValue: 'value1' },
      { id: '2', tagName: 'Prio2', tagValue: 'value2' },
      { id: '3', tagName: 'Other2', tagValue: 'value3' },
      { id: '4', tagName: 'Prio1', tagValue: 'value4' }
    ]

    const result = sortTagFieldsByRelevance(fields, ['Prio1', 'Prio2'], false)

    expect(result.map((f) => f.tagName)).toEqual(['Prio1', 'Prio2', 'Other1', 'Other2'])
  })

  it('ascending: places non-priority tags before priority tags', () => {
    const fields: TagField[] = [
      { id: '1', tagName: 'Other1', tagValue: 'value1' },
      { id: '2', tagName: 'Prio2', tagValue: 'value2' },
      { id: '3', tagName: 'Other2', tagValue: 'value3' },
      { id: '4', tagName: 'Prio1', tagValue: 'value4' }
    ]

    const result = sortTagFieldsByRelevance(fields, ['Prio1', 'Prio2'], true)

    expect(result.map((f) => f.tagName)).toEqual(['Other1', 'Other2', 'Prio2', 'Prio1'])
  })
})
