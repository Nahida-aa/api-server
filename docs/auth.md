## base

### login

url: `/api/login`

requestMethod: `POST`

requestBody:
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| username | string | 用户名 |
| password | string | 密码 |

responseBody:

200：
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| token | string | 访问令牌 |
| expires | int | 过期时间 (秒) |

404：
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| error | string | 用户不存在 |

401：
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| error | string | 密码或用户名错误 |

```http
POST http://localhost:8000/api/login
Content-Type: application/json

{
  "username": "aa",
  "password": "aaa"
}
```

### sginup
url: `/api/sginup`

requestMethod: `POST`

requestBody:
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| username | string | 用户名 |
| password | string | 密码 |

responseBody:

200：
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| token | string | 访问令牌 |
| expires | int | 过期时间 (秒) |

409：
| 参数 | 类型 | 描述 |
| ---- | ---- | ---- |
| error | string | 用户名已存在 |