export const addNote = (noteList: string[], newNote: string) => {
  noteList.push(newNote)
  return noteList
}

export const removeNote = (noteList: string[], index: number) => {
  return noteList.splice(index, 1)
}

export const updateNote = (noteList: string[], index: number, data: string) => {
  noteList[index] = data
  return noteList
}