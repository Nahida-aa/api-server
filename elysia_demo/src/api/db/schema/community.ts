import { AnyPgColumn, boolean, index, integer, jsonb, pgTable, text, uniqueIndex, uuid, varchar } from "drizzle-orm/pg-core";
import { uuidWithTimestamps } from "../columnsHelpers";
import { user } from "./auth-schema";

// 社区表 - 类似Discord 服务器(server\guild)，与项目/组织1对1对应
export const community = pgTable("community", {
  ...uuidWithTimestamps,
  name: varchar('name', { length: 255 }).notNull(),
  summary: varchar('summary', { length: 500 }),
  image: text(),

  // 1对1关系：每个项目或组织对应一个社区
  type: varchar('type', { length: 20 }).notNull(), // project, organization
  entityId: varchar('entity_id', { length: 255 }).notNull(), // 项目ID或组织ID
  defaultChannelId: uuid('default_channel_id'),

  // 社区设置
  isPublic: boolean('is_public').default(true).notNull(),
  verificationLevel: integer('verification_level').default(0).notNull(), // 0-4, 类似Discord验证等级
  memberCount: integer('member_count').default(1).notNull(),
  // 创建者
  ownerId: varchar('owner_id', { length: 255 }).notNull().references(() => user.id),
}, (table) => [
  uniqueIndex("community_entity_unique_idx").on(table.type, table.entityId), // 确保1对1
  index("community_entity_idx").on(table.type, table.entityId),
  index("community_public_idx").on(table.isPublic),
]);
// 频道表 - 属于社区
export const channel = pgTable("channel", {
  ...uuidWithTimestamps,
  communityId: uuid('community_id').references(() => community.id, { onDelete: 'cascade' }),

  name: varchar('name', { length: 255 }).notNull(),
  description: text('description'),

  // 频道类型 不按照 discord 的分类
  type: varchar('type', { length: 20 }).default('chat').notNull(), // readme{简介}, forum, welcome, announcement, guide{可以是其他人提供的攻略}, release{发布版本,用户侧}, DM{私信, 有真正的成员表}
  // parentId: uuid('parent_id').references(() => channel.id), // 分类频道ID grouping

  // 排序位置
  position: integer('position').default(0).notNull(),

  // 频道权限覆写 - 覆盖默认权限 用于给 频道的 角色或成员设置特定权限
  permissionOverwrites: jsonb('permission_overwrites').$type<Array<{
    id: string; // 角色ID或用户ID
    type: 'role' | 'member';
    allow: string; // 允许的权限位掩码（字符串形式的bigint）
    deny: string;  // 拒绝的权限位掩码
  }>>().default([]).notNull(),

  // 频道设置
  isNsfw: boolean('is_nsfw').default(false).notNull(), // 用来标记某些频道内容敏感
  rateLimitPerUser: integer('rate_limit_per_user').default(0).notNull(), // 慢速模式，秒数
}, (table) => [
  index("channel_community_idx").on(table.communityId),
  // index("channel_parent_idx").on(table.parentId),
  index("channel_position_idx").on(table.communityId, table.position),
]);

// 频道消息表
export const channelMessage = pgTable("channel_message", {
  ...uuidWithTimestamps,
  channelId: uuid('channel_id').notNull().references(() => channel.id, { onDelete: 'cascade' }),
  // userId: varchar('user_id', { length: 255 }).notNull().references(() => user.id, { onDelete: 'set null' }), // 消息不应该因为 用户删除而删除
  userId: varchar('user_id', { length: 255 }).notNull().references(() => user.id), // 消息不应该因为 用户删除而删除

  content: text('content'),
  contentType: varchar('content_type', { length: 20 }).default('text').notNull(),

  // 消息关联
  replyToId: uuid('reply_to_id').references((): AnyPgColumn => channelMessage.id),
  // threadId: uuid('thread_id').references(() => message.id), // TODO: 未来可能考虑的功能

  // 消息状态
  isEdited: boolean('is_edited').default(false).notNull(),
  isDeleted: boolean('is_deleted').default(false).notNull(),
  isPinned: boolean('is_pinned').default(false).notNull(),

  // 消息数据
  attachments: jsonb('attachments').$type<Array<{
    id: string;
    name: string;
    url: string;
    type: string;
    size: number;
  }>>().default([]).notNull(),

  mentions: jsonb('mentions').$type<{
    users?: string[];
    roles?: string[];
    channels?: string[];
    everyone?: boolean;
  }>().default({}).notNull(),

  reactions: jsonb('reactions').$type<Array<{
    emoji: string;
    count: number;
    users: string[];
  }>>().default([]).notNull(),
}, (table) => [
  index("channel_message_channel_idx").on(table.channelId),
  index("channel_message_user_idx").on(table.userId),
  // index("message_thread_idx").on(table.threadId),
  index("channel_message_created_at_idx").on(table.createdAt),
]);