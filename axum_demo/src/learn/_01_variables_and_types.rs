// 01 - 变量与类型
// 对比 C/Python/TypeScript 的变量声明

#[allow(dead_code)]
pub fn run() {
    println!("=== 01. 变量与类型 ===");
    
    // 1. 变量声明 - 默认不可变 (与 C\Python 不同)
    let x = 5;
    println!("不可变变量 x: {}", x);
    // x = 6; // 错误！默认不可变; cannot mutate immutable variable `x`
    
    // 2. 可变变量 - 需要显式声明
    let mut y = 5;
    println!("可变变量 y: {}", y);
    y = 6; // OK
    println!("修改后的 y: {}", y);
    
    // 3. 类型推断 (类似 TypeScript)
    let auto_int = 42;           // 自动推断为 i32
    let auto_float = 3.14;       // 自动推断为 f64
    let auto_bool = true;        // 自动推断为 bool
    let auto_str = "hello";      // 自动推断为 &str
    
    println!("自动推断: {} {} {} {}", auto_int, auto_float, auto_bool, auto_str);
    
    // 4. 显式类型注解 (类似 TypeScript)
    let explicit_int: i32 = 42;
    let explicit_float: f64 = 3.14;
    let explicit_char: char = '🦀';
    
    println!("显式类型: {} {} {}", explicit_int, explicit_float, explicit_char);
    
    // 5. 变量遮蔽 (Shadowing) - Rust 特有
    let spaces = "   ";          // &str 类型
    let spaces = spaces.len();   // usize 类型，遮蔽了前面的变量
    println!("遮蔽后的 spaces: {}", spaces);
    
    // 6. 常见数值类型
    let tiny: i8 = 127;          // 8位有符号整数
    let small: i16 = 32_767;     // 16位有符号整数
    let normal: i32 = 2_147_483_647; // 32位有符号整数（默认）
    let big: i64 = 9_223_372_036_854_775_807; // 64位有符号整数
    let huge: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    
    println!("整数类型: {} {} {} {} {}", tiny, small, normal, big, huge);
    
    // 7. 浮点类型
    let float32: f32 = 3.14159;  // 32位浮点
    let float64: f64 = 2.718281828; // 64位浮点（默认）
    
    println!("浮点类型: {} {}", float32, float64);
    
    // 8. 字符类型 - Unicode 标量值（4字节）
    let heart = '❤';
    let chinese = '中';
    let emoji = '🚀';
    
    println!("字符类型: {} {} {}", heart, chinese, emoji);
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 基本变量声明
    let name = "Rust";
    let version = 1.70;
    let is_awesome = true;
    
    println!("语言: {}, 版本: {}, 很棒吗: {}", name, version, is_awesome);
    
    // 可变变量示例
    let mut count = 0;
    for i in 1..=5 {
        count += i;
        println!("第 {} 次累加，当前值: {}", i, count);
    }
}

/*
与其他语言对比：

C 语言：
int x = 5;           // 可变
const int y = 10;    // 不可变

Python：
x = 5                # 可变（变量本身可重新赋值）
# Python 没有真正的不可变变量

TypeScript：
let x = 5;           // 可变
const y = 10;        // 不可变

Rust：
let x = 5;           // 不可变（默认）
let mut y = 10;      // 可变
const Z: i32 = 15;   // 编译时常量
*/
