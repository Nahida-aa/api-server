import { defineConfig } from "drizzle-kit";

export default defineConfig({
  schema: "./src/api/db/schema/*",
  out: "./drizzle",
  dialect: "postgresql",
  dbCredentials: {
    url: process.env.DATABASE_URL || "postgresql://localhost:5432/chat_db",
  },
});
