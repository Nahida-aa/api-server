import { pgTable, text, timestamp, boolean, integer, varchar } from "drizzle-orm/pg-core";

export const user = pgTable("user", {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	email: text('email').notNull().unique(),
	emailVerified: boolean('email_verified').$defaultFn(() => false).notNull(),
	image: text('image'),
	createdAt: timestamp('created_at').$defaultFn(() => /* @__PURE__ */ new Date()).notNull(),
	updatedAt: timestamp('updated_at').$defaultFn(() => /* @__PURE__ */ new Date()).notNull(),
	twoFactorEnabled: boolean('two_factor_enabled'),
	username: text('username').notNull().unique(),
	displayUsername: text('display_username'),
	isAnonymous: boolean('is_anonymous'),
	phoneNumber: text('phone_number').unique(),
	phoneNumberVerified: boolean('phone_number_verified'),
	role: text('role'), // 例如：user, admin
	banned: boolean('banned'),
	banReason: text('ban_reason'),
  banExpires: timestamp("ban_expires"),
  summary: text("summary"),
  description: text("description"),
});