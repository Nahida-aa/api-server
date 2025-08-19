// Rust 学习演示程序
// 基于 C/Python/TypeScript 背景的系统化学习路径

mod apps;
mod learn;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        show_help();
        return;
    }
    
    match args[1].as_str() {
        "learn" => {
            if args.len() >= 3 {
                run_lesson(&args[2]);
            } else {
                show_lessons();
            }
        },
        "web" => {
            println!("启动 Web 服务器...");
            println!("提示：要运行 Web 服务器，请安装 tokio 并修改 main 函数为异步");
            println!("参考 learn::_88_web_server 模块中的完整示例");
        },
        "help" | "-h" | "--help" => show_help(),
        _ => {
            println!("未知命令: {}", args[1]);
            show_help();
        }
    }
}

fn show_help() {
    println!("Rust 学习程序");
    println!();
    println!("用法:");
    println!("  cargo run learn [课程编号]  - 运行指定课程");
    println!("  cargo run learn           - 显示所有课程");
    println!("  cargo run web             - 启动 Web 服务器");
    println!("  cargo run help            - 显示帮助");
    println!();
    println!("示例:");
    println!("  cargo run learn 01        - 运行第1课：变量与类型");
    println!("  cargo run learn 21        - 运行第21课：所有权基础");
    println!("  cargo run learn 88        - 运行第88课：Web 服务器");
}

fn show_lessons() {
    println!("=== Rust 学习路径 ===");
    println!();
    
    println!("📚 阶段一：基础语法 (01-20)");
    println!("  01 - 变量与类型           - let, mut, 类型推断");
    println!("  02 - 常量与静态变量       - const, static");
    println!("  03 - 基本操作符           - 算术、比较、逻辑操作");
    println!("  04 - 控制流               - if, loop, while, for, match");
    println!("  05 - 函数                 - fn, 参数, 返回值, 闭包");
    println!("  06-20 - 其他基础概念      - (待实现)");
    println!();
    
    println!("🔐 阶段二：所有权系统 (21-40)");
    println!("  21 - 所有权基础           - 移动语义, 克隆, 栈vs堆");
    println!("  23 - 借用与引用           - &T, &mut T, 借用规则");
    println!("  24-40 - 其他所有权概念    - (待实现)");
    println!();
    
    println!("🏗️ 阶段三：数据结构 (41-60)");
    println!("  41 - 结构体               - struct, impl, 方法");
    println!("  42 - 枚举                 - enum, match, Option, Result");
    println!("  43-60 - 其他数据结构      - (待实现)");
    println!();
    
    println!("⚡ 阶段四：高级特性 (61-80)");
    println!("  61 - 错误处理             - Result, ?, panic!, 自定义错误");
    println!("  62-80 - 其他高级特性      - (待实现)");
    println!();
    
    println!("🌐 阶段五：实用开发 (81-99)");
    println!("  84 - 异步基础             - async/await, Future, 异步概念");
    println!("  88 - Web 服务器           - Axum 框架, HTTP 服务");
    println!("  89-99 - 其他实用技能      - (待实现)");
    println!();
    
    println!("使用方法:");
    println!("  cargo run learn 01   # 运行第1课");
    println!("  cargo run learn 21   # 运行第21课");
    println!("  cargo run learn 88   # 运行第88课");
    println!();
    println!("💡 提示：每个课程都包含详细的对比说明和可运行的示例代码");
}

fn run_lesson(lesson: &str) {
    match lesson {
        "01" => {
            println!("🦀 第01课：变量与类型");
            learn::_01_variables_and_types::run();
            println!("\n--- 可运行示例 ---");
            learn::_01_variables_and_types::main_example();
        },
        "02" => {
            println!("🦀 第02课：常量与静态变量");
            learn::_02_constants_and_static::run();
            println!("\n--- 可运行示例 ---");
            learn::_02_constants_and_static::main_example();
        },
        "03" => {
            println!("🦀 第03课：基本操作符");
            learn::_03_basic_operators::run();
            println!("\n--- 可运行示例 ---");
            learn::_03_basic_operators::main_example();
        },
        "04" => {
            println!("🦀 第04课：控制流");
            learn::_04_control_flow::run();
            println!("\n--- 可运行示例 ---");
            learn::_04_control_flow::main_example();
        },
        "05" => {
            println!("🦀 第05课：函数");
            learn::_05_functions::run();
            println!("\n--- 可运行示例 ---");
            learn::_05_functions::main_example();
        },
        "21" => {
            println!("🦀 第21课：所有权基础");
            learn::_21_ownership_basics::run();
            println!("\n--- 可运行示例 ---");
            learn::_21_ownership_basics::main_example();
        },
        "23" => {
            println!("🦀 第23课：借用与引用");
            learn::_23_borrowing::run();
            println!("\n--- 可运行示例 ---");
            learn::_23_borrowing::main_example();
        },
        "41" => {
            println!("🦀 第41课：结构体");
            learn::_41_structs::run();
            println!("\n--- 可运行示例 ---");
            learn::_41_structs::main_example();
        },
        "42" => {
            println!("🦀 第42课：枚举");
            learn::_42_enums::run();
            println!("\n--- 可运行示例 ---");
            learn::_42_enums::main_example();
        },
        "61" => {
            println!("🦀 第61课：错误处理");
            learn::_61_error_handling::run();
            println!("\n--- 可运行示例 ---");
            learn::_61_error_handling::main_example();
        },
        "84" => {
            println!("🦀 第84课：异步基础");
            learn::_84_async_basics::run();
            println!("\n--- 可运行示例 ---");
            learn::_84_async_basics::main_example();
        },
        "88" => {
            println!("🦀 第88课：Web 服务器");
            learn::_88_web_server::run();
            println!("\n--- 可运行示例 ---");
            learn::_88_web_server::main_example();
        },
        _ => {
            println!("❌ 课程 {} 暂未实现", lesson);
            println!("可用课程: 01, 02, 03, 04, 05, 21, 23, 41, 42, 61, 84, 88");
            println!("运行 'cargo run learn' 查看完整课程列表");
        }
    }
}