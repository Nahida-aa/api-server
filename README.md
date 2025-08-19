# web_server_demo
```bash
# pgsql create web_demo database; archlinux
# 创建本地数据库
createdb web_demo
# 检查
psql -l
# 连接 url: postgres://{owner_name}@localhost:5432/web_demo
# 连接 url: postgres://aa@localhost:5432/web_demo
```

| 框架 / 库      | 源码大小（MB） | 主要组成                                |
| ----------- | -------- | ----------------------------------- |
| **React**   | 0.5–2    | 组件、hooks、context、路由（React Router）   |
| **Vue**     | 0.5–2    | 组件、composables、router、store         |
| **Svelte**  | 0.5–2    | 组件、stores、router（svelte-kit）        |
| **Next.js** | 2–10     | pages/app、React 组件、API routes、配置    |
| **Nuxt**    | 2–10     | pages/components/composables、模板、配置  |
| **Hono**    | <1       | 路由、handler、schemas                  |
| **Elysia**  | <1       | 路由、schemas、插件                       |
| **FastAPI** | <1       | Python 代码、Pydantic schemas、路由       |
| **Django**  | 1–5      | settings、urls、migrations、模板         |
| **Gin**     | <1       | Go handler + routing                |
| **Axum**    | <1       | Rust handler + routing + Cargo.toml |
| **Spring**  | 3–10     | Java 代码 + resources + 配置            |

## 命名

| name | description | main |
| :--: | :--------- | :--: |
| schema | 结构\模型, 数据库结构, 数据验证结构, 数据转换结构, 数据输入输出结构 |
| model | 模型\结构, 数据库模型, 数据验证模型, 数据转换模型, 数据输入输出模型 | 建议当作: io_model |
| table | 数据库表, 数据库表结构\模型 | sql_table |
| route | 路由, 路由路径, 路由处理函数  | method+path |
| handler | 处理函数, 路由处理函数, 请求处理函数 | f: (req) => res |
| router | 路由器, 接收 `route` 调用对应的 `handler` | route -> handler |
| request | 请求, 请求对象, 请求数据 | req |
| response | 响应, 响应对象, 响应数据 | res |
| service | 业务服务, 业务逻辑处理函数, 用于纵向解耦 `handler` | f |

```ts
app.post(path, handler)
```

## Server API Framework

- [ ] router
- [ ] openapi docs
- [ ] crud
- [ ] db
- [ ] auth

### next

### Hone

### elysia

### fastapi

### nuxt

### sveltekit

### axum

```bash
# install rust
## arch linux
sudo pacman -S rustup
rustup default stable
rustc --version
cargo --version
# create axum project
cargo new axum_demo
cd axum_demo
```

### gin

```bash
# 创建文件夹
mkdir gin_demo
cd gin_demo
# 检查版本 (optional)
go version
# init go module
go mod init gin_demo
# install gin
go get -u github.com/gin-gonic/gin
```
```go file=gin_demo/main.go
package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func main() {
	// 创建一个默认的 Gin 引擎，内置了 Logger 和 Recovery 中间件
	r := gin.Default()

	// 定义一个 GET 路由
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	// 启动服务，监听 8080 端口
	r.Run(":8080")
}
```

### spring

## Client UI Framework

### react

### vue

### svelte

