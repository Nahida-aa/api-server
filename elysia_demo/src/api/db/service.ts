import { and, desc, eq } from "drizzle-orm";
import { db } from "./index";
import { channel, channelMessage, community, user } from "./schema";

// 社区相关操作
export const communityService = {
  // 获取所有公开社区
  async getPublicCommunities() {
    return await db.select().from(community).where(eq(community.isPublic, true));
  },

  // 根据ID获取社区
  async getCommunityById(id: string) {
    const [result] = await db.select().from(community).where(eq(community.id, id));
    return result;
  },

  // 创建社区
  async createCommunity(data: {
    name: string;
    summary?: string;
    type: string;
    entityId: string;
    ownerId: string;
    isPublic?: boolean;
  }) {
    const [result] = await db.insert(community).values(data).returning();
    return result;
  },
};

// 频道相关操作
export const channelService = {
  // 获取社区下的所有频道
  async getChannelsByCommunity(communityId: string) {
    return await db
      .select()
      .from(channel)
      .where(eq(channel.communityId, communityId))
      .orderBy(channel.position);
  },

  // 根据ID获取频道
  async getChannelById(id: string) {
    const [result] = await db.select().from(channel).where(eq(channel.id, id));
    return result;
  },

  // 创建频道
  async createChannel(data: {
    communityId: string;
    name: string;
    description?: string;
    type?: string;
    position?: number;
  }) {
    const [result] = await db.insert(channel).values(data).returning();
    return result;
  },

  // 获取或创建默认频道
  async getOrCreateDefaultChannel(communityId: string) {
    let channels = await this.getChannelsByCommunity(communityId);

    if (channels.length === 0) {
      const defaultChannel = await this.createChannel({
        communityId,
        name: "general",
        description: "默认频道",
        type: "chat",
        position: 0,
      });
      return defaultChannel;
    }

    return channels[0];
  },
};

// 消息相关操作
export const messageService = {
  // 获取频道消息（分页）
  async getChannelMessages(channelId: string, limit = 50, offset = 0) {
    return await db
      .select({
        id: channelMessage.id,
        channelId: channelMessage.channelId,
        userId: channelMessage.userId,
        content: channelMessage.content,
        contentType: channelMessage.contentType,
        createdAt: channelMessage.createdAt,
        updatedAt: channelMessage.updatedAt,
        isEdited: channelMessage.isEdited,
        replyToId: channelMessage.replyToId,
        attachments: channelMessage.attachments,
        mentions: channelMessage.mentions,
        reactions: channelMessage.reactions,
        user: {
          id: user.id,
          name: user.name,
          username: user.username,
          displayUsername: user.displayUsername,
          image: user.image,
        },
      })
      .from(channelMessage)
      .leftJoin(user, eq(channelMessage.userId, user.id))
      .where(and(
        eq(channelMessage.channelId, channelId),
        eq(channelMessage.isDeleted, false)
      ))
      .orderBy(desc(channelMessage.createdAt))
      .limit(limit)
      .offset(offset);
  },

  // 发送消息
  async createMessage(data: {
    channelId: string;
    userId: string;
    content: string;
    contentType?: string;
    replyToId?: string;
  }) {
    const [result] = await db.insert(channelMessage).values({
      ...data,
      contentType: data.contentType || 'text',
    }).returning();
    return result;
  },

  // 删除消息（软删除）
  async deleteMessage(messageId: string, userId: string) {
    const [result] = await db
      .update(channelMessage)
      .set({ isDeleted: true, updatedAt: new Date() })
      .where(and(
        eq(channelMessage.id, messageId),
        eq(channelMessage.userId, userId)
      ))
      .returning();
    return result;
  },

  // 编辑消息
  async editMessage(messageId: string, userId: string, content: string) {
    const [result] = await db
      .update(channelMessage)
      .set({
        content,
        isEdited: true,
        updatedAt: new Date()
      })
      .where(and(
        eq(channelMessage.id, messageId),
        eq(channelMessage.userId, userId)
      ))
      .returning();
    return result;
  },
};

// 用户相关操作
export const userService = {
  // 根据ID获取用户
  async getUserById(id: string) {
    const [result] = await db.select().from(user).where(eq(user.id, id));
    return result;
  },

  // 创建或更新用户
  async upsertUser(data: {
    id: string;
    name: string;
    email: string;
    username: string;
    image?: string;
  }) {
    const [result] = await db
      .insert(user)
      .values(data)
      .onConflictDoUpdate({
        target: user.id,
        set: {
          name: data.name,
          email: data.email,
          username: data.username,
          image: data.image,
          updatedAt: new Date(),
        },
      })
      .returning();
    return result;
  },
};
