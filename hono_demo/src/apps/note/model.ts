import { createInsertSchema, createSelectSchema, createUpdateSchema } from "drizzle-typebox";
import { noteTable } from "./table";

export const noteInsertSchema = createInsertSchema(noteTable);
export const noteUpdateSchema = createUpdateSchema(noteTable);
export const noteSelectSchema = createSelectSchema(noteTable);