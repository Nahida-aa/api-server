import { Elysia, t } from 'elysia';
import { setChannelUsersGetter } from '../community/router';
import { channelService, messageService, userService } from '../db/service';

// 消息类型定义
export interface Message {
  id: string;
  channelId: string;
  userId: string;
  content?: string | null;
  contentType: string;
  replyToId?: string | null;
  attachments?: {
    id: string;
    name: string;
    url: string;
    type: string;
    size: number;
  }[] | null;
  createdAt: Date;
  user?: {
    id: string;
    username: string;
    displayUsername?: string;
    image: string | null;
  };
}

// 频道用户信息
export interface ChannelUser {
  userId: string;
  username: string;
  joinedAt: Date;
}

// 客户端发送的消息类型
export const clientWsDataT = t.Union([
  t.Object({
    op: t.Literal("joinChannel"),
    d: t.Object({
      channelId: t.String(),
      userId: t.String(),
      username: t.String()
    })
  }),
  t.Object({
    op: t.Literal("leaveChannel"), // 离开频道但保持订阅
    d: t.Object({
      channelId: t.String(),
      userId: t.String()
    })
  }),
  t.Object({
    op: t.Literal("unsubscribeChannel"), // 完全退订频道
    d: t.Object({
      channelId: t.String(),
      userId: t.String()
    })
  })
])
// clientWsDataT
// t -> type
export type ClientWsData = typeof clientWsDataT.static

// 服务器发送的消息类型
export type ServerWsData = {
  op: "newMessage";
  d: Message;
} | {
  op: "userJoined";
  d: {
    channelId: string;
    user: ChannelUser;
  };
} | {
  op: "userLeft";  // 用户离开频道的实时通知 left 是 leave 的过去式
  d: {
    channelId: string;
    userId: string;
  };
} | {
  op: "error";
  d: {
    message: string;
  };
}

// 使用 Map 存储聊天数据（函数式设计）
const activeChannels = new Map<string, Set<string>>(); // channelId -> Set of ws connection ids (正在查看频道的用户)
const subscribedChannels = new Map<string, Set<string>>(); // channelId -> Set of ws connection ids (订阅频道的用户)
const userChannels = new Map<string, string>(); // ws connection id -> active channelId
const channelUsers = new Map<string, Map<string, ChannelUser>>(); // channelId -> userId -> user info (活跃用户)
const messages = new Map<string, Message[]>(); // channelId -> messages
const wsConnections = new Map<string, any>(); // connection id -> ws


const removeConnection = (connectionId: string) => {
  const channelId = userChannels.get(connectionId);
  if (channelId) {
    // WebSocket 断开时完全退订频道
    unsubscribeChannel(connectionId, channelId);
  }
  wsConnections.delete(connectionId);
  userChannels.delete(connectionId);
};

// 广播函数
const broadcastToChannel = (channelId: string, data: ServerWsData, onlyActive = false) => {
  // 如果只向活跃用户广播（如用户加入/离开消息）
  if (onlyActive) {
    const connections = activeChannels.get(channelId);
    if (connections) {
      for (const connectionId of connections) {
        const ws = wsConnections.get(connectionId);
        if (ws) {
          try {
            ws.send(data);
          } catch (error) {
            console.error(`Failed to send message to connection ${connectionId}:`, error);
            removeConnection(connectionId);
          }
        }
      }
    }
  } else {
    // 向所有订阅用户广播（如新消息）
    const subscribedConnections = subscribedChannels.get(channelId) || new Set();
    const activeConnections = activeChannels.get(channelId) || new Set();
    const allConnections = new Set([...subscribedConnections, ...activeConnections]);

    for (const connectionId of allConnections) {
      const ws = wsConnections.get(connectionId);
      if (ws) {
        try {
          ws.send(data);
        } catch (error) {
          console.error(`Failed to send message to connection ${connectionId}:`, error);
          removeConnection(connectionId);
        }
      }
    }
  }
};

// 频道管理函数
const initChannel = (channelId: string) => {
  if (!activeChannels.has(channelId)) {
    activeChannels.set(channelId, new Set());
    subscribedChannels.set(channelId, new Set());
    channelUsers.set(channelId, new Map());
    messages.set(channelId, []);
  }
};

const joinChannel = (connectionId: string, channelId: string, userId: string, username: string) => {
  // 如果用户已经在其他频道，先自动离开当前频道
  const currentChannel = userChannels.get(connectionId);
  if (currentChannel && currentChannel !== channelId) {
    leaveChannel(connectionId, currentChannel);
  }

  // 初始化频道数据
  initChannel(channelId);

  // 添加用户到活跃频道
  activeChannels.get(channelId)!.add(connectionId);
  // 也添加到订阅列表，确保用户能收到消息
  subscribedChannels.get(channelId)!.add(connectionId);
  userChannels.set(connectionId, channelId);

  const user: ChannelUser = {
    userId,
    username,
    joinedAt: new Date()
  };
  channelUsers.get(channelId)!.set(userId, user);

  // 广播用户加入消息（只向活跃用户广播）
  broadcastToChannel(channelId, {
    op: "userJoined",
    d: { channelId, user }
  }, true); // 第三个参数为 true，表示只向活跃用户广播

  // 不再发送用户列表，客户端应该通过 HTTP API 获取
};

const leaveChannel = (connectionId: string, channelId: string) => {
  const activeChannel = activeChannels.get(channelId);
  if (!activeChannel) return;

  // 从活跃频道移除，但保留在订阅列表中
  activeChannel.delete(connectionId);

  // 找到用户ID并从活跃用户列表移除
  const channelUserMap = channelUsers.get(channelId);
  let userId: string | undefined;

  if (channelUserMap) {
    for (const [uid, user] of channelUserMap) {
      if (userChannels.get(connectionId) === channelId) {
        userId = uid;
        channelUserMap.delete(uid);
        break;
      }
    }
  }

  userChannels.delete(connectionId);

  // 广播用户离开消息（只向活跃用户广播）
  if (userId) {
    broadcastToChannel(channelId, {
      op: "userLeft",
      d: { channelId, userId }
    }, true); // 只向活跃用户广播
  }

  // 如果频道没有活跃用户了，清理活跃频道数据
  if (activeChannel.size === 0) {
    activeChannels.delete(channelId);
    channelUsers.delete(channelId);
    // 保留订阅列表和消息历史
  }
};

const unsubscribeChannel = (connectionId: string, channelId: string) => {
  // 从活跃频道和订阅列表中完全移除
  const activeChannel = activeChannels.get(channelId);
  const subscribedChannel = subscribedChannels.get(channelId);

  if (activeChannel) {
    activeChannel.delete(connectionId);
  }

  if (subscribedChannel) {
    subscribedChannel.delete(connectionId);
  }

  // 如果这是用户当前活跃的频道，清理相关数据
  if (userChannels.get(connectionId) === channelId) {
    const channelUserMap = channelUsers.get(channelId);
    let userId: string | undefined;

    if (channelUserMap) {
      for (const [uid, user] of channelUserMap) {
        if (userChannels.get(connectionId) === channelId) {
          userId = uid;
          channelUserMap.delete(uid);
          break;
        }
      }
    }

    userChannels.delete(connectionId);

    // 广播用户离开消息
    if (userId) {
      broadcastToChannel(channelId, {
        op: "userLeft",
        d: { channelId, userId }
      }, true);
    }
  }

  // 清理空的频道数据
  if (activeChannel && activeChannel.size === 0) {
    activeChannels.delete(channelId);
    channelUsers.delete(channelId);
  }

  if (subscribedChannel && subscribedChannel.size === 0) {
    subscribedChannels.delete(channelId);
    // 可以考虑清理消息历史，但通常保留
  }
};

// 消息处理函数
const sendMessage = async (channelId: string, userId: string, content: string, contentType: string, replyToId?: string) => {
  try {
    // 验证频道是否存在
    const channel = await channelService.getChannelById(channelId);
    if (!channel) {
      throw new Error("Channel not found");
    }

    // 验证用户是否存在
    const user = await userService.getUserById(userId);
    if (!user) {
      throw new Error("User not found");
    }

    // 保存消息到数据库
    const dbMessage = await messageService.createMessage({
      channelId,
      userId,
      content,
      contentType,
      replyToId
    });

    // 构造带用户信息的消息
    const message: Message = {
      id: dbMessage.id,
      channelId: dbMessage.channelId,
      userId: dbMessage.userId,
      content: dbMessage.content,
      contentType: dbMessage.contentType,
      replyToId: dbMessage.replyToId,
      attachments: dbMessage.attachments || [],
      createdAt: dbMessage.createdAt,
      user: {
        id: user.id,
        username: user.username,
        displayUsername: user.displayUsername || undefined,
        image: user.image,
      }
    };

    // 广播消息（发送给所有订阅用户，包括不在当前频道的用户）
    broadcastToChannel(channelId, {
      op: "newMessage",
      d: message
    }, false); // false 表示发送给所有订阅用户

    return message;
  } catch (error) {
    console.error("Error sending message:", error);
    throw error;
  }
};

// 查询函数
const getChannelMessages = async (channelId: string, limit: number = 50): Promise<Message[]> => {
  try {
    const dbMessages = await messageService.getChannelMessages(channelId, limit);
    return dbMessages.map(msg => ({
      id: msg.id,
      channelId: msg.channelId,
      userId: msg.userId,
      content: msg.content,
      contentType: msg.contentType,
      replyToId: msg.replyToId,
      attachments: msg.attachments || [],
      createdAt: msg.createdAt,
      user: msg.user ? {
        id: msg.user.id,
        name: msg.user.name,
        username: msg.user.username,
        displayUsername: msg.user.displayUsername || undefined,
        image: msg.user.image || null,
      } : undefined
    }));
  } catch (error) {
    console.error("Error getting channel messages:", error);
    return [];
  }
};

const getChannelUsers = (channelId: string): ChannelUser[] => {
  const users = channelUsers.get(channelId);
  return users ? Array.from(users.values()) : [];
};

// 注册获取在线用户的函数到社区路由
setChannelUsersGetter(getChannelUsers);

export const wsApp = new Elysia()
  .ws('/ws', {
    body: clientWsDataT,
    open(ws) {
      const connectionId = crypto.randomUUID();
      (ws as any).connectionId = connectionId;
      wsConnections.set(connectionId, ws)
      console.log(`WebSocket connected: ${connectionId}`);
    },

    message(ws, message) {
      const connectionId = (ws as any).connectionId;
      if (!connectionId) {
        ws.send({
          op: "error",
          d: { message: "Connection not properly initialized" }
        });
        return;
      }

      // 异步处理消息, 客户端通过 http post 发送消息
      (async () => {
        try {
          const data = message;

          switch (data.op) {
            case "joinChannel":
              joinChannel(
                connectionId,
                data.d.channelId,
                data.d.userId,
                data.d.username
              );
              break;
            case "leaveChannel":
              // 用户离开频道但保持订阅（仍能收到新消息通知）
              // 通常用于用户切换到其他页面但仍想收到频道消息
              leaveChannel(connectionId, data.d.channelId);
              break;
            case "unsubscribeChannel":
              // 用户完全退订频道（不再接收任何消息）
              // 通常用于用户明确表示不想再收到该频道的任何信息
              unsubscribeChannel(connectionId, data.d.channelId);
              break;
            default:
              ws.send({
                op: "error",
                d: { message: "Unknown operation" }
              });
          }
        } catch (error) {
          console.error("WebSocket message error:", error);
          ws.send({
            op: "error",
            d: { message: error instanceof Error ? error.message : "Invalid message format" }
          });
        }
      })();
    },
    close(ws) {
      const connectionId = (ws as any).connectionId;
      if (connectionId) {
        removeConnection(connectionId);
        console.log(`WebSocket disconnected: ${connectionId}`);
      }
    }
  });