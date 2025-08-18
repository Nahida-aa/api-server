import { Elysia, t } from 'elysia'
import { db } from '../../db';
import { noteTable } from './table';
import { eq } from 'drizzle-orm';
import { noteInsertSchema, noteSelectSchema, noteUpdateSchema } from './model';

export const noteApp = new Elysia({
    detail: {
      tags: ['Note'],
      description: 'Note management API'
    }
  })
  .get('/note', async () => {
    return await db.select().from(noteTable);
  },{
    response: t.Array(noteSelectSchema)
  })
  .put('/note', async ({ body }) => {
    const createdNote = await db.insert(noteTable).values(body).returning();
    return createdNote[0]
  }, 
  {
    body: t.Omit(noteInsertSchema, ['id']),
    response: noteSelectSchema
  })
  .get('/note/:id', async ({ params: { id }}) => {
    const selectedNote = await db.select().from(noteTable).where(eq(noteTable.id, id));
    return selectedNote[0];
  },
  {
    params: t.Object({
      id: t.String()
    }),
    response: noteSelectSchema
  })
  .delete('/note/:id', async ({ params: { id }}) => {
    const deletedNote = await db.delete(noteTable).where(eq(noteTable.id, id)).returning();
    return deletedNote[0];
  },
  {
    params: t.Object({
      id: t.String()
    }),
    response: noteSelectSchema
  })
  .patch('/note/:id', async ({ params: { id }, body }) => {
    const updatedNote = await db.update(noteTable).set(body).where(eq(noteTable.id, id)).returning();
    return updatedNote[0]
  },
  {
    params: t.Object({
      id: t.String()
    }),
    body: t.Omit(noteUpdateSchema, ['id']),
    response: noteSelectSchema
  }) 