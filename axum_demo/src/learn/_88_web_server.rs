// 88 - Web 服务器
// 使用 Axum 构建现代 Web 服务器

#[allow(dead_code)]
pub fn run() {
    println!("=== 88. Web 服务器 ===");
    
    // 这里展示 Web 服务器的概念和代码结构
    web_server_concepts();
    axum_basics();
    routing_examples();
    middleware_examples();
    error_handling_web();
    complete_server_example();
}

fn web_server_concepts() {
    println!("--- Web 服务器基础概念 ---");
    
    println!("HTTP 服务器组件:");
    println!("1. 路由 (Router) - 将 URL 映射到处理函数");
    println!("2. 处理器 (Handler) - 处理具体的请求");
    println!("3. 中间件 (Middleware) - 在请求/响应间执行逻辑");
    println!("4. 状态 (State) - 共享应用状态");
    println!("5. 提取器 (Extractor) - 从请求中提取数据");
    println!();
    
    println!("Axum 特点:");
    println!("- 基于 tokio 的异步框架");
    println!("- 类型安全的提取器");
    println!("- 零成本抽象");
    println!("- 优秀的错误处理");
    println!("- 丰富的中间件生态");
}

fn axum_basics() {
    println!("--- Axum 基础 ---");
    
    println!("基本的 Axum 服务器结构:");
    println!();
    println!("// Cargo.toml 依赖");
    println!("[dependencies]");
    println!("axum = \"0.7\"");
    println!("tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    println!("tower = \"0.4\"");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("serde_json = \"1.0\"");
    println!();
    
    println!("// 基本服务器");
    println!("use axum::{{");
    println!("    extract::{{Path, Query}},");
    println!("    http::StatusCode,");
    println!("    response::{{Html, Json}},");
    println!("    routing::{{get, post}},");
    println!("    Router,");
    println!("}};");
    println!("use std::collections::HashMap;");
    println!();
    
    println!("async fn hello() -> &'static str {{");
    println!("    \"Hello, World!\"");
    println!("}}");
    println!();
    
    println!("#[tokio::main]");
    println!("async fn main() {{");
    println!("    let app = Router::new()");
    println!("        .route(\"/\", get(hello));");
    println!();
    println!("    let listener = tokio::net::TcpListener::bind(\"127.0.0.1:3000\")");
    println!("        .await.unwrap();");
    println!("    ");
    println!("    println!(\"服务器启动在 http://127.0.0.1:3000\");");
    println!("    axum::serve(listener, app).await.unwrap();");
    println!("}}");
}

fn routing_examples() {
    println!("--- 路由示例 ---");
    
    println!("// 路径参数");
    println!("async fn get_user(Path(user_id): Path<u32>) -> String {{");
    println!("    format!(\"用户 ID: {{}}\", user_id)");
    println!("}}");
    println!();
    
    println!("// 查询参数");
    println!("async fn search(Query(params): Query<HashMap<String, String>>) -> String {{");
    println!("    format!(\"搜索参数: {{:?}}\", params)");
    println!("}}");
    println!();
    
    println!("// JSON 请求体");
    println!("#[derive(serde::Deserialize)]");
    println!("struct CreateUser {{");
    println!("    name: String,");
    println!("    email: String,");
    println!("}}");
    println!();
    println!("async fn create_user(Json(payload): Json<CreateUser>) -> Json<CreateUser> {{");
    println!("    Json(payload)");
    println!("}}");
    println!();
    
    println!("// 路由组合");
    println!("let app = Router::new()");
    println!("    .route(\"/\", get(hello))");
    println!("    .route(\"/users/:id\", get(get_user))");
    println!("    .route(\"/search\", get(search))");
    println!("    .route(\"/users\", post(create_user))");
    println!("    .nest(\"/api/v1\", api_routes())");
    println!("    .nest_service(\"/static\", tower_http::services::ServeDir::new(\"static\"));");
}

fn middleware_examples() {
    println!("--- 中间件示例 ---");
    
    println!("// 日志中间件");
    println!("use tower_http::trace::TraceLayer;");
    println!();
    println!("let app = Router::new()");
    println!("    .route(\"/\", get(hello))");
    println!("    .layer(TraceLayer::new_for_http());");
    println!();
    
    println!("// CORS 中间件");
    println!("use tower_http::cors::{{CorsLayer, Origin}};");
    println!();
    println!("let cors = CorsLayer::new()");
    println!("    .allow_origin(Origin::exact(\"http://localhost:3000\".parse().unwrap()))");
    println!("    .allow_methods([Method::GET, Method::POST])");
    println!("    .allow_headers([CONTENT_TYPE]);");
    println!();
    
    println!("let app = Router::new()");
    println!("    .route(\"/api/data\", get(get_data))");
    println!("    .layer(cors);");
    println!();
    
    println!("// 自定义中间件");
    println!("use axum::middleware;");
    println!();
    println!("async fn auth_middleware<B>(");
    println!("    request: Request<B>,");
    println!("    next: Next<B>,");
    println!(") -> Result<Response, StatusCode> {{");
    println!("    let auth_header = request.headers()");
    println!("        .get(\"authorization\")");
    println!("        .and_then(|header| header.to_str().ok());");
    println!();
    println!("    match auth_header {{");
    println!("        Some(token) if token.starts_with(\"Bearer \") => {{");
    println!("            Ok(next.run(request).await)");
    println!("        }}");
    println!("        _ => Err(StatusCode::UNAUTHORIZED),");
    println!("    }}");
    println!("}}");
    println!();
    
    println!("let protected_routes = Router::new()");
    println!("    .route(\"/profile\", get(get_profile))");
    println!("    .layer(middleware::from_fn(auth_middleware));");
}

fn error_handling_web() {
    println!("--- Web 错误处理 ---");
    
    println!("// 自定义错误类型");
    println!("#[derive(Debug)]");
    println!("enum AppError {{");
    println!("    NotFound,");
    println!("    BadRequest(String),");
    println!("    InternalServerError,");
    println!("}}");
    println!();
    
    println!("impl IntoResponse for AppError {{");
    println!("    fn into_response(self) -> Response {{");
    println!("        let (status, error_message) = match self {{");
    println!("            AppError::NotFound => (StatusCode::NOT_FOUND, \"未找到资源\"),");
    println!("            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),");
    println!("            AppError::InternalServerError => (");
    println!("                StatusCode::INTERNAL_SERVER_ERROR,");
    println!("                \"内部服务器错误\"");
    println!("            ),");
    println!("        }};");
    println!();
    println!("        let body = Json(json!({{");
    println!("            \"error\": error_message");
    println!("        }}));");
    println!();
    println!("        (status, body).into_response()");
    println!("    }}");
    println!("}}");
    println!();
    
    println!("// 使用自定义错误");
    println!("async fn get_user_by_id(Path(id): Path<u32>) -> Result<Json<User>, AppError> {{");
    println!("    if id == 0 {{");
    println!("        return Err(AppError::BadRequest(\"ID 不能为 0\".to_string()));");
    println!("    }}");
    println!();
    println!("    // 模拟数据库查询");
    println!("    if id > 1000 {{");
    println!("        return Err(AppError::NotFound);");
    println!("    }}");
    println!();
    println!("    Ok(Json(User {{");
    println!("        id,");
    println!("        name: format!(\"用户{{}}\", id),");
    println!("        email: format!(\"user{{}}@example.com\", id),");
    println!("    }}))");
    println!("}}");
}

fn complete_server_example() {
    println!("--- 完整服务器示例 ---");
    
    println!("这是一个完整的 Axum Web 服务器示例:");
    println!();
    println!("// src/main.rs");
    println!("use axum::{{");
    println!("    extract::{{Path, Query, State}},");
    println!("    http::StatusCode,");
    println!("    middleware,");
    println!("    response::{{Html, IntoResponse, Json}},");
    println!("    routing::{{get, post, put, delete}},");
    println!("    Router,");
    println!("}};");
    println!("use serde::{{Deserialize, Serialize}};");
    println!("use std::{{");
    println!("    collections::HashMap,");
    println!("    sync::{{Arc, Mutex}},");
    println!("}};");
    println!("use tower_http::{{cors::CorsLayer, trace::TraceLayer}};");
    println!();
    
    println!("// 数据模型");
    println!("#[derive(Debug, Clone, Serialize, Deserialize)]");
    println!("struct User {{");
    println!("    id: u32,");
    println!("    name: String,");
    println!("    email: String,");
    println!("}}");
    println!();
    
    println!("#[derive(Deserialize)]");
    println!("struct CreateUserRequest {{");
    println!("    name: String,");
    println!("    email: String,");
    println!("}}");
    println!();
    
    println!("// 应用状态");
    println!("type UserStore = Arc<Mutex<HashMap<u32, User>>>;");
    println!();
    
    println!("#[derive(Clone)]");
    println!("struct AppState {{");
    println!("    users: UserStore,");
    println!("    next_id: Arc<Mutex<u32>>,");
    println!("}}");
    println!();
    
    println!("// 路由处理器");
    println!("async fn health_check() -> &'static str {{");
    println!("    \"服务器运行正常\"");
    println!("}}");
    println!();
    
    println!("async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {{");
    println!("    let users = state.users.lock().unwrap();");
    println!("    let user_list: Vec<User> = users.values().cloned().collect();");
    println!("    Json(user_list)");
    println!("}}");
    println!();
    
    println!("async fn get_user(");
    println!("    Path(id): Path<u32>,");
    println!("    State(state): State<AppState>,");
    println!(") -> Result<Json<User>, StatusCode> {{");
    println!("    let users = state.users.lock().unwrap();");
    println!("    match users.get(&id) {{");
    println!("        Some(user) => Ok(Json(user.clone())),");
    println!("        None => Err(StatusCode::NOT_FOUND),");
    println!("    }}");
    println!("}}");
    println!();
    
    println!("async fn create_user(");
    println!("    State(state): State<AppState>,");
    println!("    Json(request): Json<CreateUserRequest>,");
    println!(") -> Result<Json<User>, StatusCode> {{");
    println!("    let mut next_id = state.next_id.lock().unwrap();");
    println!("    let id = *next_id;");
    println!("    *next_id += 1;");
    println!("    drop(next_id);");
    println!();
    println!("    let user = User {{");
    println!("        id,");
    println!("        name: request.name,");
    println!("        email: request.email,");
    println!("    }};");
    println!();
    println!("    let mut users = state.users.lock().unwrap();");
    println!("    users.insert(id, user.clone());");
    println!();
    println!("    Ok(Json(user))");
    println!("}}");
    println!();
    
    println!("async fn delete_user(");
    println!("    Path(id): Path<u32>,");
    println!("    State(state): State<AppState>,");
    println!(") -> StatusCode {{");
    println!("    let mut users = state.users.lock().unwrap();");
    println!("    match users.remove(&id) {{");
    println!("        Some(_) => StatusCode::NO_CONTENT,");
    println!("        None => StatusCode::NOT_FOUND,");
    println!("    }}");
    println!("}}");
    println!();
    
    println!("#[tokio::main]");
    println!("async fn main() {{");
    println!("    // 初始化日志");
    println!("    tracing_subscriber::init();");
    println!();
    println!("    // 创建应用状态");
    println!("    let state = AppState {{");
    println!("        users: Arc::new(Mutex::new(HashMap::new())),");
    println!("        next_id: Arc::new(Mutex::new(1)),");
    println!("    }};");
    println!();
    println!("    // 构建路由");
    println!("    let app = Router::new()");
    println!("        .route(\"/\", get(|| async {{ \"欢迎使用 Rust Web API!\" }}))");
    println!("        .route(\"/health\", get(health_check))");
    println!("        .route(\"/users\", get(get_users).post(create_user))");
    println!("        .route(\"/users/:id\", get(get_user).delete(delete_user))");
    println!("        .with_state(state)");
    println!("        .layer(CorsLayer::permissive())");
    println!("        .layer(TraceLayer::new_for_http());");
    println!();
    println!("    // 启动服务器");
    println!("    let listener = tokio::net::TcpListener::bind(\"127.0.0.1:3000\")");
    println!("        .await");
    println!("        .unwrap();");
    println!();
    println!("    println!(\"🚀 服务器启动在 http://127.0.0.1:3000\");");
    println!("    println!(\"📋 可用端点:\");");
    println!("    println!(\"  GET  /           - 欢迎页面\");");
    println!("    println!(\"  GET  /health     - 健康检查\");");
    println!("    println!(\"  GET  /users      - 获取所有用户\");");
    println!("    println!(\"  POST /users      - 创建用户\");");
    println!("    println!(\"  GET  /users/:id  - 获取指定用户\");");
    println!("    println!(\"  DELETE /users/:id - 删除用户\");");
    println!();
    println!("    axum::serve(listener, app).await.unwrap();");
    println!("}}");
    println!();
    
    println!("API 使用示例:");
    println!("# 获取所有用户");
    println!("curl http://localhost:3000/users");
    println!();
    println!("# 创建用户");
    println!("curl -X POST http://localhost:3000/users \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{\"name\": \"张三\", \"email\": \"zhangsan@example.com\"}}'");
    println!();
    println!("# 获取指定用户");
    println!("curl http://localhost:3000/users/1");
    println!();
    println!("# 删除用户");
    println!("curl -X DELETE http://localhost:3000/users/1");
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    println!("=== Web 服务器快速开始 ===");
    println!();
    println!("要创建一个简单的 Web 服务器，请按以下步骤操作:");
    println!();
    println!("1. 在 Cargo.toml 中添加依赖:");
    println!("[dependencies]");
    println!("axum = \"0.7\"");
    println!("tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("serde_json = \"1.0\"");
    println!("tower = \"0.4\"");
    println!("tower-http = {{ version = \"0.5\", features = [\"full\"] }}");
    println!("tracing = \"0.1\"");
    println!("tracing-subscriber = \"0.3\"");
    println!();
    println!("2. 将上面的完整示例代码复制到 src/main.rs");
    println!();
    println!("3. 运行服务器:");
    println!("cargo run");
    println!();
    println!("4. 测试 API:");
    println!("curl http://localhost:3000/health");
    println!();
    
    show_advanced_features();
}

fn show_advanced_features() {
    println!("--- 高级特性 ---");
    
    println!("1. WebSocket 支持:");
    println!("use axum::{{");
    println!("    extract::{{ws::{{Message, WebSocket, WebSocketUpgrade}}}},");
    println!("    response::Response,");
    println!("}};");
    println!();
    println!("async fn websocket_handler(ws: WebSocketUpgrade) -> Response {{");
    println!("    ws.on_upgrade(handle_socket)");
    println!("}}");
    println!();
    
    println!("2. 文件上传:");
    println!("use axum::extract::Multipart;");
    println!();
    println!("async fn upload(mut multipart: Multipart) {{");
    println!("    while let Some(field) = multipart.next_field().await.unwrap() {{");
    println!("        let name = field.name().unwrap().to_string();");
    println!("        let data = field.bytes().await.unwrap();");
    println!("        println!(\"上传文件: {{}}, 大小: {{}}\", name, data.len());");
    println!("    }}");
    println!("}}");
    println!();
    
    println!("3. 数据库集成 (SQLx):");
    println!("use sqlx::{{PgPool, Row}};");
    println!();
    println!("async fn get_users_from_db(");
    println!("    State(pool): State<PgPool>");
    println!(") -> Result<Json<Vec<User>>, StatusCode> {{");
    println!("    let users = sqlx::query!(\"SELECT id, name, email FROM users\")");
    println!("        .fetch_all(&pool)");
    println!("        .await");
    println!("        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;");
    println!("    ");
    println!("    // 转换为 User 结构体...");
    println!("    Ok(Json(vec![]))  // 简化示例");
    println!("}}");
    println!();
    
    println!("4. JWT 认证:");
    println!("use jsonwebtoken::{{decode, DecodingKey, Validation}};");
    println!();
    println!("async fn verify_jwt(");
    println!("    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,");
    println!(") -> Result<Json<Claims>, StatusCode> {{");
    println!("    let token = authorization.token();");
    println!("    let claims = decode::<Claims>(");
    println!("        token,");
    println!("        &DecodingKey::from_secret(\"secret\".as_ref()),");
    println!("        &Validation::default(),");
    println!("    )");
    println!("    .map_err(|_| StatusCode::UNAUTHORIZED)?;");
    println!("    ");
    println!("    Ok(Json(claims.claims))");
    println!("}}");
    println!();
    
    println!("Axum 的优势:");
    println!("✅ 类型安全的提取器");
    println!("✅ 优秀的性能");
    println!("✅ 丰富的中间件生态");
    println!("✅ WebSocket 支持");
    println!("✅ 灵活的路由系统");
    println!("✅ 与 tokio 生态系统完美集成");
}

/*
Web 框架对比：

Express.js (Node.js):
- 简单易用
- 中间件丰富
- 单线程限制

Spring Boot (Java):
- 功能完整
- 重量级框架
- 启动时间长

FastAPI (Python):
- 类型提示
- 自动文档生成
- GIL 限制性能

Django (Python):
- 全功能框架
- ORM 集成
- 性能有限

Gin (Go):
- 高性能
- 简洁 API
- 并发友好

Axum (Rust) 优势：
✅ 类型安全 - 编译时检查
✅ 高性能 - 零成本抽象
✅ 内存安全 - 所有权系统
✅ 并发安全 - 无数据竞争
✅ 生态丰富 - tokio 生态系统
✅ 现代设计 - async/await 原生支持

核心特性：
1. 基于 Tower 的中间件系统
2. 类型安全的请求提取
3. 灵活的响应类型
4. 状态共享机制
5. 嵌套路由支持
6. WebSocket 和 SSE 支持
*/
