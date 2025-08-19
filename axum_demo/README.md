# Rust 学习路径

基于 C/Python/TypeScript 背景的 Rust 系统化学习教程。

## 🎯 学习目标

针对已有 C、Python、TypeScript 和 Kotlin 经验的开发者，提供系统化的 Rust 学习路径。每个示例都包含与熟悉语言的对比，帮助快速理解 Rust 的独特之处。

## 📚 课程结构

### 阶段一：基础语法 (01-20)
- ✅ **01 - 变量与类型** - let, mut, 类型推断
- ✅ **02 - 常量与静态变量** - const, static
- ✅ **03 - 基本操作符** - 算术、比较、逻辑操作
- ✅ **04 - 控制流** - if, loop, while, for, match
- ✅ **05 - 函数** - fn, 参数, 返回值, 闭包
- 🚧 **06-20** - 其他基础概念 (待实现)

### 阶段二：所有权系统 (21-40)
- ✅ **21 - 所有权基础** - 移动语义, 克隆, 栈vs堆
- ✅ **23 - 借用与引用** - &T, &mut T, 借用规则
- 🚧 **22, 24-40** - 其他所有权概念 (待实现)

### 阶段三：数据结构 (41-60)
- ✅ **41 - 结构体** - struct, impl, 方法
- ✅ **42 - 枚举** - enum, match, Option, Result
- 🚧 **43-60** - 其他数据结构 (待实现)

### 阶段四：高级特性 (61-80)
- ✅ **61 - 错误处理** - Result, ?, panic!, 自定义错误
- 🚧 **62-80** - 其他高级特性 (待实现)

### 阶段五：实用开发 (81-99)
- ✅ **84 - 异步基础** - async/await, Future, 异步概念
- ✅ **88 - Web 服务器** - Axum 框架, HTTP 服务
- 🚧 **81-83, 85-87, 89-99** - 其他实用技能 (待实现)

## 🚀 快速开始

### 1. 克隆项目
```bash
git clone <repository-url>
cd axum_demo
```

### 2. 查看所有课程
```bash
cargo run learn
```

### 3. 运行指定课程
```bash
# 基础语法
cargo run learn 01  # 变量与类型
cargo run learn 02  # 常量与静态变量
cargo run learn 03  # 基本操作符
cargo run learn 04  # 控制流
cargo run learn 05  # 函数

# 所有权系统 (Rust 核心概念)
cargo run learn 21  # 所有权基础
cargo run learn 23  # 借用与引用

# 数据结构
cargo run learn 41  # 结构体
cargo run learn 42  # 枚举

# 高级特性
cargo run learn 61  # 错误处理

# 实用开发
cargo run learn 84  # 异步基础
cargo run learn 88  # Web 服务器
```

### 4. 查看帮助
```bash
cargo run help
```

## 💡 学习建议

### 对于有 C 背景的开发者
- 重点关注：**所有权系统** (21-40) - 这是与 C 最大的不同
- 对比学习：内存管理方式的差异
- 优势理解：编译时内存安全保证

### 对于有 Python 背景的开发者
- 重点关注：**类型系统** (01-20) - 静态类型 vs 动态类型
- 对比学习：错误处理方式的差异 (61课)
- 优势理解：性能提升和类型安全

### 对于有 TypeScript 背景的开发者
- 重点关注：**所有权和借用** (21-40) - 比 TS 更严格的内存管理
- 对比学习：枚举和模式匹配 (42课)
- 优势理解：零成本抽象和并发安全

### 对于有 Kotlin 背景的开发者
- 重点关注：**异步编程** (84课) - async/await 的不同实现
- 对比学习：函数式编程特性
- 优势理解：无垃圾回收的内存管理

## 📖 每课内容

每个课程包含：

1. **概念讲解** - 核心概念和原理
2. **语言对比** - 与 C/Python/TypeScript/Kotlin 的对比
3. **实际示例** - 可运行的代码示例
4. **最佳实践** - 实际开发中的建议
5. **常见陷阱** - 需要避免的错误

## 🛠️ 实际项目

### Web 服务器 (课程88)
```bash
# 查看 Web 服务器示例
cargo run learn 88

# 要运行实际的 Web 服务器，需要添加依赖到 Cargo.toml:
# [dependencies]
# axum = "0.7"
# tokio = { version = "1.0", features = ["full"] }
# serde = { version = "1.0", features = ["derive"] }
# tower = "0.4"
# tower-http = "0.5"
```

## 🔍 代码组织

```
src/
├── main.rs              # 主程序入口
├── apps/                # 应用模块
│   ├── mod.rs
│   └── note/            # 笔记应用示例
└── learn/               # 学习模块
    ├── mod.rs           # 模块声明
    ├── _01_variables_and_types.rs
    ├── _02_constants_and_static.rs
    ├── _03_basic_operators.rs
    ├── _04_control_flow.rs
    ├── _05_functions.rs
    ├── _21_ownership_basics.rs
    ├── _23_borrowing.rs
    ├── _41_structs.rs
    ├── _42_enums.rs
    ├── _61_error_handling.rs
    ├── _84_async_basics.rs
    └── _88_web_server.rs
```

## 🎓 学习进度跟踪

建议按以下顺序学习：

### 第一周：基础语法
- [ ] 01 - 变量与类型
- [ ] 02 - 常量与静态变量  
- [ ] 03 - 基本操作符
- [ ] 04 - 控制流
- [ ] 05 - 函数

### 第二周：所有权系统 (核心)
- [ ] 21 - 所有权基础
- [ ] 23 - 借用与引用

### 第三周：数据结构
- [ ] 41 - 结构体
- [ ] 42 - 枚举

### 第四周：错误处理与实用特性
- [ ] 61 - 错误处理
- [ ] 84 - 异步基础
- [ ] 88 - Web 服务器

## 🤝 贡献

欢迎提交 PR 来完善课程内容：

1. 添加新的课程 (06-20, 22, 24-40, 43-60, 62-83, 85-87, 89-99)
2. 改进现有课程的示例
3. 添加更多语言对比
4. 修复错误和改进文档

## 📚 推荐资源

- [官方 Rust 教程](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings 练习](https://github.com/rust-lang/rustlings)
- [Axum 文档](https://docs.rs/axum/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)

## 📄 许可证

MIT License

---

🦀 **Happy Rust Learning!** 

从熟悉的语言背景出发，系统掌握 Rust 的核心概念和实践技能。
