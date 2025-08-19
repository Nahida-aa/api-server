// 21 - 所有权基础
// Rust 最重要的概念：所有权系统

#[allow(dead_code)]
pub fn run() {
    println!("=== 21. 所有权基础 ===");
    
    // 1. 所有权规则演示
    ownership_rules();
    
    // 2. 移动语义
    move_semantics();
    
    // 3. 克隆数据
    clone_data();
    
    // 4. 栈 vs 堆数据
    stack_vs_heap();
}

fn ownership_rules() {
    println!("--- 所有权规则 ---");
    
    // 规则1: Rust 中的每一个值都有一个所有者
    let s1 = String::from("hello");
    println!("s1 的值: {}", s1);
    // s1 是字符串 "hello" 的所有者
    
    // 规则2: 值在任一时刻有且只有一个所有者
    // let s2 = s1;  // 所有权从 s1 移动到 s2
    // println!("{}", s1);  // 错误！s1 不再有效
    
    // 规则3: 当所有者离开作用域，这个值将被丢弃
    {
        let s3 = String::from("world");
        println!("s3 在内部作用域: {}", s3);
    } // s3 在这里离开作用域并被自动释放
    
    // println!("{}", s3);  // 错误！s3 已经不存在了
}

fn move_semantics() {
    println!("--- 移动语义 ---");
    
    // 对于栈数据，实现了 Copy trait，会自动复制
    let x = 5;
    let y = x;  // 复制，不是移动
    println!("x: {}, y: {} (都有效，因为 i32 实现了 Copy)", x, y);
    
    // 对于堆数据，默认是移动
    let s1 = String::from("hello");
    let s2 = s1;  // 移动！s1 不再有效
    
    println!("s2: {}", s2);
    // println!("s1: {}", s1);  // 错误！s1 已被移动
    
    // 函数调用也会发生移动
    let s3 = String::from("world");
    takes_ownership(s3);  // s3 的所有权移动到函数中
    // println!("s3: {}", s3);  // 错误！s3 已被移动
    
    // 函数可以返回所有权
    let s4 = gives_ownership();
    println!("从函数获得的所有权: {}", s4);
    
    let s5 = String::from("test");
    let s6 = takes_and_gives_back(s5);  // s5 移动进去，新值移动出来
    println!("取回的所有权: {}", s6);
    // println!("s5: {}", s5);  // 错误！s5 已被移动
}

fn takes_ownership(some_string: String) {
    println!("函数接收所有权: {}", some_string);
} // some_string 在这里离开作用域并被释放

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 返回所有权给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回传入的字符串
}

fn clone_data() {
    println!("--- 克隆数据 ---");
    
    // 使用 clone() 创建深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝，两个都有效
    
    println!("s1: {}, s2: {} (两个都有效)", s1, s2);
    
    // 对于简单的栈数据，自动复制
    let x = 5;
    let y = x;  // 自动复制（实现了 Copy trait）
    println!("x: {}, y: {}", x, y);
    
    // 实现了 Copy 的类型：
    // - 所有整数类型 (i32, u32, i64, etc.)
    // - 布尔类型 bool
    // - 所有浮点类型 (f32, f64)
    // - 字符类型 char
    // - 仅包含 Copy 类型的元组
    
    let tuple1 = (1, 2.0, true);
    let tuple2 = tuple1;  // 复制
    println!("tuple1: {:?}, tuple2: {:?}", tuple1, tuple2);
}

fn stack_vs_heap() {
    println!("--- 栈 vs 堆 ---");
    
    // 栈数据：已知固定大小，访问快速，自动管理
    let stack_int = 42;              // 存储在栈上
    let stack_array = [1, 2, 3, 4]; // 存储在栈上
    let stack_tuple = (1, 2); // 存储在栈上
    
    println!("栈数据 - 整数: {}, 数组: {:?}, 元组: {:?}", 
                    stack_int, stack_array, stack_tuple);
    
    // 堆数据：大小可变或编译时未知，通过指针访问
    let heap_string = String::from("stored on heap"); // 字符串内容在堆上
    let heap_vector = vec![1, 2, 3, 4]; // 向量内容在堆上
    
    println!("堆数据 - 字符串: {}, 向量: {:?}", heap_string, heap_vector);
    
    // 显示内存地址差异
    println!("栈变量地址: {:p}", &stack_int);
    println!("堆字符串指针地址: {:p}", &heap_string);
    println!("堆字符串内容地址: {:p}", heap_string.as_ptr());
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 所有权的实际应用示例
    
    // 1. 安全的字符串操作
    fn process_string(s: String) -> String {
        let processed = format!("处理后的: {}", s.to_uppercase());
        processed
    }
    
    let original = String::from("hello rust");
    let result = process_string(original);
    println!("结果: {}", result);
    // original 已经被移动，不能再使用
    
    // 2. 避免所有权问题的方法 - 使用引用（下节课内容预告）
    fn process_string_ref(s: &str) -> String {
        format!("处理后的: {}", s.to_uppercase())
    }
    
    let original2 = String::from("hello world");
    let result2 = process_string_ref(&original2);  // 借用，不移动所有权
    println!("结果: {}", result2);
    println!("原始字符串仍然有效: {}", original2);  // 仍然可以使用
    
    // 3. 向量的所有权
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    
    let numbers2 = numbers;  // 移动
    // numbers.push(5);      // 错误！numbers 已被移动
    
    println!("移动后的向量: {:?}", numbers2);
    
    // 4. 所有权与函数返回值
    fn create_vector() -> Vec<i32> {
        let v = vec![1, 2, 3, 4, 5];
        v  // 返回所有权
    }
    
    let my_vec = create_vector();
    println!("从函数获得的向量: {:?}", my_vec);
}

/*
所有权系统 vs 其他语言的内存管理：

C 语言：
- 手动内存管理 (malloc/free)
- 容易出现内存泄漏、悬空指针、双重释放
- 性能高但不安全

Java/Kotlin：
- 垃圾回收器自动管理
- 安全但有性能开销
- 可能出现内存泄漏（循环引用）

Python：
- 引用计数 + 垃圾回收
- 简单易用但性能开销大

TypeScript/JavaScript：
- 垃圾回收器
- 开发简单但运行时开销

Rust 的所有权系统：
✅ 内存安全：编译时防止内存错误
✅ 零运行时开销：没有垃圾回收器
✅ 无需手动管理：自动释放内存
✅ 并发安全：防止数据竞争

核心概念：
1. 每个值都有唯一的所有者
2. 当所有者离开作用域时，值被自动释放
3. 移动语义转移所有权
4. Clone 可以创建深拷贝
5. Copy trait 允许自动复制简单类型
*/
