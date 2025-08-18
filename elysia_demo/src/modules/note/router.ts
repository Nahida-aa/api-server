import { Elysia, t } from 'elysia'
import { addNote, removeNote, updateNote } from './service'

const noteList = ['Moonhalo']

export const noteApp = new Elysia()
  .decorate('noteList', noteList)
  .get('/note', ({ noteList }) => noteList)
  .put(
    '/note', 
    ({ noteList, body: { newNote } }) => addNote(noteList, newNote), 
    {
      body: t.Object({
        newNote: t.String()
      })
    }
  )
  .get(
    '/note/:index',
    ({ noteList, params: { index }, status }) => {
      return noteList[index] ?? status(404, 'Not Found :(')
    },
    {
      params: t.Object({
        index: t.Number()
      })
    }
  )
  .delete(
    '/note/:index',
    ({ noteList, params: { index }, status }) => {
      if (index in noteList) return removeNote(noteList, index)

      return status(422)
    },
    {
      params: t.Object({
        index: t.Number()
      })
    }
  )
  .patch(
    '/note/:index',
    ({ noteList, params: { index }, body: { newNote }, status }) => {
      if (index in noteList) return updateNote(noteList, index, newNote)

      return status(422)
    },
    {
      params: t.Object({
        index: t.Number()
      }),
      body: t.Object({
        newNote: t.String()
      })
    }
  ) 