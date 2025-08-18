import { integer, pgTable, uuid, varchar } from "drizzle-orm/pg-core";

export const noteTable = pgTable("note", {
  id: uuid("id").primaryKey().defaultRandom(),
  title: varchar("title", { length: 255 }).notNull(),
  content: varchar("content", { length: 2048 }).notNull()
});