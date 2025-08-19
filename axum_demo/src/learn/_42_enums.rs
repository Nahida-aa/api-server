// 42 - 枚举
// 强大的代数数据类型，远超其他语言的枚举

#[allow(dead_code)]
pub fn run() {
    println!("=== 42. 枚举 ===");
    
    // 1. 基本枚举
    basic_enums();
    
    // 2. 带数据的枚举
    enums_with_data();
    
    // 3. Option 枚举
    option_enum();
    
    // 4. Result 枚举  
    result_enum();
    
    // 5. 枚举方法
    enum_methods();
    
    // 6. 模式匹配
    pattern_matching();
}

// 1. 基本枚举 - 类似其他语言的枚举
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// 2. 带数据的枚举 - Rust 的强大特性
#[derive(Debug)]
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },   // 匿名结构体
    Write(String),              // 元组
    ChangeColor(i32, i32, i32), // 元组
}

// 3. 网络地址枚举
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 4. Web 事件枚举
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn basic_enums() {
    println!("--- 基本枚举 ---");
    
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    
    for direction in directions {
        match direction {
            Direction::North => println!("向北走"),
            Direction::South => println!("向南走"),
            Direction::East => println!("向东走"),
            Direction::West => println!("向西走"),
        }
    }
}

fn enums_with_data() {
    println!("--- 带数据的枚举 ---");
    
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for message in messages {
        process_message(message);
    }
    
    // IP 地址示例
    let addresses = vec![
        IpAddr::V4(127, 0, 0, 1),
        IpAddr::V6(String::from("::1")),
        IpAddr::V4(192, 168, 1, 1),
    ];
    
    for addr in addresses {
        println!("IP 地址: {:?}", addr);
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出程序"),
        Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
        Message::Write(text) => println!("写入文本: {}", text),
        Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
    }
}

fn option_enum() {
    println!("--- Option 枚举 ---");
    
    // Option<T> 用于表示可能为空的值，替代 null
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("有值的数字: {:?}", some_number);
    println!("有值的字符串: {:?}", some_string);
    println!("空的数字: {:?}", absent_number);
    
    // 使用 Option 的各种方法
    let x = Some(5);
    let y: Option<i32> = None;
    
    // unwrap_or: 提供默认值
    println!("x.unwrap_or(0): {}", x.unwrap_or(0));
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));
    
    // map: 对 Some 中的值应用函数
    let doubled = x.map(|v| v * 2);
    println!("x 乘以 2: {:?}", doubled);
    
    // and_then: 链式操作
    let result = x.and_then(|v| {
        if v > 0 { Some(v * 10) } else { None }
    });
    println!("链式操作结果: {:?}", result);
    
    // 实际应用：查找数组中的元素
    let numbers = vec![1, 3, 5, 7, 9];
    let found = find_element(&numbers, 5);
    
    match found {
        Some(index) => println!("找到元素 5 在位置 {}", index),
        None => println!("未找到元素 5"),
    }
}

fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn result_enum() {
    println!("--- Result 枚举 ---");
    
    // Result<T, E> 用于错误处理
    let good_result: Result<i32, &str> = Ok(10);
    let bad_result: Result<i32, &str> = Err("出错了！");
    
    println!("成功结果: {:?}", good_result);
    println!("错误结果: {:?}", bad_result);
    
    // 使用 Result 的方法
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 ÷ 2 = {}", result),
        Err(error) => println!("除法错误: {}", error),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 ÷ 0 = {}", result),
        Err(error) => println!("除法错误: {}", error),
    }
    
    // unwrap_or_else: 错误时执行函数
    let result = divide(15.0, 3.0).unwrap_or_else(|_| 0.0);
    println!("除法结果或默认值: {}", result);
    
    // map_err: 转换错误类型
    let converted = divide(8.0, 2.0).map_err(|e| format!("计算错误: {}", e));
    println!("转换后的结果: {:?}", converted);
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("调用退出消息"),
            Message::Move { x, y } => println!("调用移动消息到 ({}, {})", x, y),
            Message::Write(text) => println!("调用写入消息: {}", text),
            Message::ChangeColor(r, g, b) => println!("调用颜色改变消息: RGB({}, {}, {})", r, g, b),
        }
    }
}

impl WebEvent {
    fn process(&self) {
        match self {
            WebEvent::PageLoad => println!("页面加载事件"),
            WebEvent::PageUnload => println!("页面卸载事件"),
            WebEvent::KeyPress(c) => println!("按键事件: '{}'", c),
            WebEvent::Paste(s) => println!("粘贴事件: '{}'", s),
            WebEvent::Click { x, y } => println!("点击事件在坐标 ({}, {})", x, y),
        }
    }
}

fn enum_methods() {
    println!("--- 枚举方法 ---");
    
    let msg = Message::Write(String::from("Hello from enum method!"));
    msg.call();
    
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("clipboard content")),
        WebEvent::Click { x: 100, y: 200 },
        WebEvent::PageUnload,
    ];
    
    for event in events {
        event.process();
    }
}

fn pattern_matching() {
    println!("--- 模式匹配 ---");
    
    // if let - 简化的 match
    let some_value = Some(3);
    
    if let Some(x) = some_value {
        println!("if let 匹配到值: {}", x);
    }
    
    // while let - 循环中的模式匹配
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    // 复杂模式匹配
    let msg = Message::ChangeColor(255, 128, 0);
    
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x: 0, y } => println!("水平不动，垂直移动到 {}", y),
        Message::Move { x, y: 0 } => println!("垂直不动，水平移动到 {}", x),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) if text.len() > 10 => println!("长文本: {}", text),
        Message::Write(text) => println!("短文本: {}", text),
        Message::ChangeColor(r, g, b) if r > 200 => println!("高红色分量: RGB({}, {}, {})", r, g, b),
        Message::ChangeColor(r, g, b) => println!("普通颜色: RGB({}, {}, {})", r, g, b),
    }
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 实际应用示例
    
    // 1. HTTP 状态码
    #[derive(Debug)]
    enum HttpStatus {
        Ok,
        NotFound,
        ServerError(String),
        Redirect { location: String },
    }
    
    impl HttpStatus {
        fn to_code(&self) -> u16 {
            match self {
                HttpStatus::Ok => 200,
                HttpStatus::NotFound => 404,
                HttpStatus::ServerError(_) => 500,
                HttpStatus::Redirect { .. } => 302,
            }
        }
        
        fn description(&self) -> String {
            match self {
                HttpStatus::Ok => "请求成功".to_string(),
                HttpStatus::NotFound => "页面未找到".to_string(),
                HttpStatus::ServerError(msg) => format!("服务器错误: {}", msg),
                HttpStatus::Redirect { location } => format!("重定向到: {}", location),
            }
        }
    }
    
    let statuses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::ServerError("数据库连接失败".to_string()),
        HttpStatus::Redirect { location: "/home".to_string() },
    ];
    
    for status in statuses {
        println!("状态码 {}: {}", status.to_code(), status.description());
    }
    
    // 2. 配置解析
    #[derive(Debug)]
    enum ConfigValue {
        Text(String),
        Number(i64),
        Boolean(bool),
        List(Vec<String>),
    }
    
    fn parse_config(input: &str) -> Option<ConfigValue> {
        if input == "true" || input == "false" {
            Some(ConfigValue::Boolean(input == "true"))
        } else if let Ok(num) = input.parse::<i64>() {
            Some(ConfigValue::Number(num))
        } else if input.starts_with('[') && input.ends_with(']') {
            let items: Vec<String> = input[1..input.len()-1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            Some(ConfigValue::List(items))
        } else {
            Some(ConfigValue::Text(input.to_string()))
        }
    }
    
    let configs = ["hello", "42", "true", "[item1, item2, item3]"];
    
    for config in configs {
        if let Some(value) = parse_config(config) {
            println!("解析 '{}' -> {:?}", config, value);
        }
    }
    
    // 3. 状态机
    #[derive(Debug)]
    enum ConnectionState {
        Disconnected,
        Connecting,
        Connected { session_id: String },
        Error { code: u32, message: String },
    }
    
    impl ConnectionState {
        fn connect(&mut self, session_id: String) {
            *self = match self {
                ConnectionState::Disconnected => {
                    println!("开始连接...");
                    ConnectionState::Connecting
                },
                ConnectionState::Connecting => {
                    println!("连接成功，会话ID: {}", session_id);
                    ConnectionState::Connected { session_id }
                },
                _ => {
                    println!("当前状态无法连接");
                    return;
                }
            };
        }
        
        fn disconnect(&mut self) {
            *self = ConnectionState::Disconnected;
            println!("已断开连接");
        }
        
        fn is_connected(&self) -> bool {
            matches!(self, ConnectionState::Connected { .. })
        }
    }
    
    let mut connection = ConnectionState::Disconnected;
    println!("初始状态: {:?}", connection);
    
    connection.connect("session_123".to_string());
    println!("第一次连接后: {:?}", connection);
    
    connection.connect("session_123".to_string());
    println!("第二次连接后: {:?}", connection);
    println!("是否已连接: {}", connection.is_connected());
    
    connection.disconnect();
    println!("断开连接后: {:?}", connection);
}

/*
枚举 vs 其他语言：

C 语言：
enum Direction {
    NORTH, SOUTH, EAST, WEST
};
// 只是整数常量，无类型安全

Java：
enum Direction {
    NORTH, SOUTH, EAST, WEST;
}
// 类型安全，但功能有限

TypeScript：
enum Direction {
    North, South, East, West
}

type Message = 
    | { type: 'quit' }
    | { type: 'move', x: number, y: number }
    | { type: 'write', text: string };
// 联合类型近似，但需要判别字段

Kotlin：
sealed class Message {
    object Quit : Message()
    data class Move(val x: Int, val y: Int) : Message()
    data class Write(val text: String) : Message()
}
// 密封类提供类似功能

Python：
from enum import Enum
class Direction(Enum):
    NORTH = 1
    SOUTH = 2
    EAST = 3
    WEST = 4
// 基本枚举，无代数数据类型

Rust 枚举的强大之处：
✅ 真正的代数数据类型 (ADT)
✅ 每个变体可以携带不同类型的数据
✅ 编译时保证模式匹配的完整性
✅ 零运行时开销
✅ 与 Option 和 Result 的完美集成
✅ 强大的模式匹配支持

核心概念：
1. 枚举定义一组可能的值
2. 每个变体可以携带数据
3. match 表达式必须处理所有情况
4. Option<T> 替代空值
5. Result<T, E> 用于错误处理
6. if let 和 while let 简化模式匹配
*/
