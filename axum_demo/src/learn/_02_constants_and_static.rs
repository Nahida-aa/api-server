// 02 - 常量与静态变量
// 理解编译时常量和运行时静态变量

#[allow(dead_code)]
pub fn run() {
    println!("=== 02. 常量与静态变量 ===");
    
    // 1. 使用常量
    println!("最大用户数: {}", MAX_USERS);
    println!("π 的值: {}", PI);
    println!("应用名称: {}", APP_NAME);
    
    // 2. 使用静态变量
    unsafe {
        let counter_value = std::ptr::addr_of!(GLOBAL_COUNTER).read();
        println!("全局计数器: {}", counter_value);
        GLOBAL_COUNTER += 1;
        let new_counter_value = std::ptr::addr_of!(GLOBAL_COUNTER).read();
        println!("递增后的计数器: {}", new_counter_value);
    }
    
    // 3. 常量函数示例
    const CALCULATED: i32 = calculate_at_compile_time(10, 5);
    println!("编译时计算结果: {}", CALCULATED);
}

// 1. 常量声明 - 编译时确定，不占用运行时内存
const MAX_USERS: usize = 1000;
const PI: f64 = 3.14159265359;
const APP_NAME: &str = "AxumDemo";

// 2. 常量表达式 - 可以在编译时计算
const fn calculate_at_compile_time(a: i32, b: i32) -> i32 {
    a + b * 2
}

const RESULT: i32 = calculate_at_compile_time(10, 20);

// 3. 静态变量 - 运行时存在，有固定内存地址
static mut GLOBAL_COUNTER: i32 = 0; // 可变静态变量需要 unsafe

// 4. 静态字符串
static WELCOME_MESSAGE: &str = "欢迎使用 Rust！";

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 常量使用
    const MAX_CONNECTIONS: usize = 100;
    const TIMEOUT_SECONDS: u64 = 30;
    
    println!("服务器配置:");
    println!("  最大连接数: {}", MAX_CONNECTIONS);
    println!("  超时时间: {} 秒", TIMEOUT_SECONDS);
    
    // 使用静态变量
    println!("欢迎信息: {}", WELCOME_MESSAGE);
    
    // 编译时计算
    const BUFFER_SIZE: usize = 1024 * 8; // 8KB
    println!("缓冲区大小: {} 字节", BUFFER_SIZE);
}

/*
与其他语言对比：

C 语言：
#define MAX_SIZE 100     // 宏定义
const int PI = 3.14;     // 运行时常量
static int counter = 0;  // 静态变量

Python：
MAX_SIZE = 100           # 约定的常量（实际可变）
# Python 没有真正的编译时常量

TypeScript：
const MAX_SIZE = 100;    // 编译时常量
let counter = 0;         // 变量

Kotlin：
const val MAX_SIZE = 100 // 编译时常量
val PI = 3.14           // 运行时常量
var counter = 0         // 变量

Rust：
const MAX_SIZE: usize = 100;    // 编译时常量
static COUNTER: i32 = 0;        // 静态变量（不可变）
static mut COUNTER: i32 = 0;    // 静态变量（可变，需要 unsafe）

特点：
1. const 必须在编译时确定值
2. static 在运行时有固定内存地址
3. static mut 需要 unsafe 块访问
*/
