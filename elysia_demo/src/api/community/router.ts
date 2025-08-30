import { Elysia, t } from 'elysia';
import { channelService, messageService } from '../db/service';

// 从WebSocket路由导入在线用户管理函数
// 注意：这是一个简化的方案，在生产环境中可能需要更复杂的状态管理
let getChannelUsers: (channelId: string) => Array<{ userId: string, username: string, joinedAt: Date }>;

// 导出设置函数供WebSocket路由使用
export const setChannelUsersGetter = (getter: typeof getChannelUsers) => {
  getChannelUsers = getter;
};

export const communityApp = new Elysia({ prefix: '/community' })
  // 获取所有频道
  .get('/:communityId/channels', async ({ params: { communityId } }) => {
    try {
      const channels = await channelService.getChannelsByCommunity(communityId);

      return {
        success: true,
        data: channels
      };
    } catch (error) {
      return {
        success: false,
        error: 'Failed to fetch channels'
      };
    }
  })
  // 获取单个频道信息
  .get('/channels/:channelId', async ({ params: { channelId } }) => {
    try {
      const channel = await channelService.getChannelById(channelId);
      if (!channel) {
        return {
          success: false,
          error: 'Channel not found'
        };
      }
      return {
        success: true,
        data: channel
      };
    } catch (error) {
      return {
        success: false,
        error: 'Failed to fetch channel'
      };
    }
  })
  // 获取频道消息
  .get('/channels/:channelId/messages', async ({ params: { channelId }, query }) => {
    try {
      const limit = parseInt(query.limit as string) || 50;
      const offset = parseInt(query.offset as string) || 0;

      const channel = await channelService.getChannelById(channelId);
      if (!channel) {
        return {
          success: false,
          error: 'Channel not found'
        };
      }

      const messages = await messageService.getChannelMessages(channelId, limit, offset);
      return {
        success: true,
        data: {
          channel,
          messages,
          pagination: {
            limit,
            offset,
            hasMore: messages.length === limit
          }
        }
      };
    } catch (error) {
      return {
        success: false,
        error: 'Failed to fetch messages'
      };
    }
  }, {
    query: t.Object({
      limit: t.Optional(t.String()),
      offset: t.Optional(t.String())
    })
  })
  // 获取频道在线用户 - HTTP API
  .get('/channels/:channelId/users', ({ params: { channelId } }) => {
    try {
      const users = getChannelUsers ? getChannelUsers(channelId) : [];
      return {
        success: true,
        data: {
          channelId,
          users,
          count: users.length
        }
      };
    } catch (error) {
      return {
        success: false,
        error: 'Failed to fetch channel users'
      };
    }
  })

  // 发送消息（HTTP接口）
  .post('/channels/:channelId/messages', async ({ params: { channelId }, body }) => {
    try {
      const channel = await channelService.getChannelById(channelId);
      if (!channel) {
        return {
          success: false,
          error: 'Channel not found'
        };
      }

      const message = await messageService.createMessage({
        channelId,
        userId: body.userId,
        content: body.content,
        contentType: body.contentType || 'text',
        replyToId: body.replyToId
      });

      return {
        success: true,
        data: message
      };
    } catch (error) {
      return {
        success: false,
        error: 'Failed to send message'
      };
    }
  }, {
    body: t.Object({
      userId: t.String(),
      content: t.String(),
      contentType: t.Optional(t.String()),
      replyToId: t.Optional(t.String())
    })
  });
