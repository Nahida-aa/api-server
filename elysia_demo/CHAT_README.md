# 频道聊天功能实现

## 功能特性

### 1. 实时聊天
- WebSocket 实时通信
- 多频道支持
- 用户加入/离开通知
- 消息历史记录

### 2. 频道管理
- 预设频道：综合讨论、技术交流、随机聊天
- 频道用户列表
- 频道间切换

### 3. 消息功能
- 文本消息发送
- 消息时间戳
- 用户身份标识

## API 接口

### WebSocket 接口
- **连接地址**: `ws://localhost:3000/ws`

#### 客户端发送消息格式
```typescript
// 加入频道
{
  op: "joinChannel",
  d: {
    channelId: string,
    userId: string,
    username: string
  }
}

// 离开频道
{
  op: "leaveChannel",
  d: {
    channelId: string,
    userId: string
  }
}

// 发送消息
{
  op: "sendMessage",
  d: {
    channelId: string,
    userId: string,
    content: string,
    contentType: string,
    replyToId?: string
  }
}
```

#### 服务器推送消息格式
```typescript
// 新消息
{
  op: "newMessage",
  d: {
    id: string,
    channelId: string,
    userId: string,
    content: string,
    contentType: string,
    replyToId?: string,
    timestamp: Date
  }
}

// 用户加入
{
  op: "userJoined",
  d: {
    channelId: string,
    user: {
      userId: string,
      username: string,
      joinedAt: Date
    }
  }
}

// 用户离开
{
  op: "userLeft",
  d: {
    channelId: string,
    userId: string
  }
}

// 频道用户列表
{
  op: "channelUsers",
  d: {
    channelId: string,
    users: Array<{
      userId: string,
      username: string,
      joinedAt: Date
    }>
  }
}

// 错误消息
{
  op: "error",
  d: {
    message: string
  }
}
```

### HTTP API 接口

#### 获取频道列表
- **GET** `/api/community/channels`
- **响应**: 
```json
{
  "success": true,
  "data": [
    {
      "id": "general",
      "name": "综合讨论",
      "description": "一般话题讨论",
      "createdAt": "2025-08-26T..."
    }
  ]
}
```

#### 获取频道信息
- **GET** `/api/community/channels/:channelId`

#### 获取频道消息
- **GET** `/api/community/channels/:channelId/messages?limit=50&offset=0`

#### 发送消息 (HTTP)
- **POST** `/api/community/channels/:channelId/messages`
- **请求体**:
```json
{
  "userId": "user123",
  "username": "测试用户",
  "content": "Hello World!",
  "contentType": "text",
  "replyToId": "msg_123" // 可选
}
```

## 快速开始

### 1. 启动服务器
```bash
cd /home/aa/repos/web_ls/web_demo/elysia_demo
bun run dev
```

### 2. 访问聊天界面
在浏览器中打开: `http://localhost:3000/chat`

### 3. 使用步骤
1. 输入用户ID和用户名
2. 点击"连接"按钮
3. 选择一个频道
4. 开始聊天！

## 技术架构

### 后端技术栈
- **Elysia**: 高性能 Web 框架
- **Bun**: JavaScript 运行时
- **WebSocket**: 实时通信
- **TypeScript**: 类型安全

### 数据存储
- 内存存储 (可扩展为数据库)
- 消息历史记录
- 用户会话管理

### 前端技术
- 原生 HTML/CSS/JavaScript
- WebSocket 客户端
- 响应式设计

## 扩展功能

可以进一步实现的功能：

1. **数据持久化**
   - 连接数据库 (PostgreSQL/MySQL)
   - 消息永久存储
   - 用户账户系统

2. **高级聊天功能**
   - 文件上传
   - 图片分享
   - 表情符号
   - 消息回复/引用

3. **用户管理**
   - 用户认证
   - 权限控制
   - 管理员功能

4. **频道增强**
   - 创建/删除频道
   - 私有频道
   - 频道权限

5. **实时功能**
   - 输入状态指示
   - 在线状态
   - 消息已读状态

## 部署说明

### 开发环境
```bash
bun run dev
```

### 生产环境
```bash
bun run build
bun start
```

### Docker 部署
可以创建 Dockerfile 进行容器化部署。

## 故障排除

### 常见问题

1. **WebSocket 连接失败**
   - 检查服务器是否启动
   - 确认端口 3000 未被占用
   - 检查防火墙设置

2. **消息发送失败**
   - 确认已连接到服务器
   - 确认已加入频道
   - 检查浏览器控制台错误

3. **频道切换问题**
   - 确保先建立 WebSocket 连接
   - 检查频道ID是否正确

### 调试技巧
- 打开浏览器开发者工具查看 WebSocket 消息
- 查看服务器控制台日志
- 使用 Swagger UI 测试 HTTP API: `http://localhost:3000/swagger`
