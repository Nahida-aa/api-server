// 23 - 借用与引用
// 解决所有权转移问题的核心机制

#[allow(dead_code)]
pub fn run() {
    println!("=== 23. 借用与引用 ===");
    
    // 1. 不可变借用
    immutable_borrowing();
    
    // 2. 可变借用
    mutable_borrowing();
    
    // 3. 借用规则
    borrowing_rules();
    
    // 4. 引用的作用域
    reference_scope();
}

fn immutable_borrowing() {
    println!("--- 不可变借用 ---");
    
    let s1 = String::from("hello");
    
    // 创建引用，借用 s1 的值而不获取所有权
    let len = calculate_length(&s1);
    
    println!("字符串 '{}' 的长度是 {}", s1, len);
    // s1 仍然有效，因为我们只是借用了它
    
    // 可以多次不可变借用
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    
    println!("多次不可变借用: {}, {}, {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s 离开作用域，但因为它只是引用，不会释放指向的数据
}

fn mutable_borrowing() {
    println!("--- 可变借用 ---");
    
    let mut s = String::from("hello");
    
    // 可变借用，允许修改借用的值
    change(&mut s);
    
    println!("修改后的字符串: {}", s);
    
    // 可变借用的限制：同一时间只能有一个可变借用
    let r1 = &mut s;
    // let r2 = &mut s;  // 错误！不能同时有两个可变借用
    
    // 使用可变借用
    r1.push_str(" world");
    println!("通过可变引用修改: {}", r1);
    
    // r1 的作用域结束后，可以创建新的借用
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn borrowing_rules() {
    println!("--- 借用规则演示 ---");
    
    let mut s = String::from("hello");
    
    // 规则1: 可以有多个不可变引用
    {
        let r1 = &s;
        let r2 = &s;
        println!("不可变引用: {} 和 {}", r1, r2);
        // r1 和 r2 的作用域在这里结束
    }
    
    // 规则2: 或者一个可变引用
    {
        let r3 = &mut s;
        r3.push_str(" world");
        println!("可变引用: {}", r3);
        // r3 的作用域在这里结束
    }
    
    // 规则3: 不能同时有可变和不可变引用
    // 下面的代码会出错：
    /*
    let r4 = &s;      // 不可变借用
    let r5 = &mut s;  // 错误！已经有不可变借用了
    println!("{} {}", r4, r5);
    */
    
    // 非词法作用域生命周期 (NLL) - 引用的生命周期在最后一次使用后结束
    let r4 = &s;
    let r5 = &s;
    println!("最后一次使用不可变引用: {} {}", r4, r5);
    // r4 和 r5 的生命周期在这里结束
    
    let r6 = &mut s;  // 现在可以创建可变引用了
    r6.push_str("!");
    println!("现在可以可变借用: {}", r6);
}

fn reference_scope() {
    println!("--- 引用作用域 ---");
    
    let mut s = String::from("hello");
    
    // 引用的生命周期从创建开始
    let r1 = &s;
    let r2 = &s;
    
    println!("{} and {}", r1, r2);
    // 变量 r1 和 r2 不会再被使用，所以生命周期在这里结束
    
    let r3 = &mut s;  // 没问题，因为 r1 和 r2 已经不再使用
    println!("{}", r3);
}

// 返回引用的函数 - 需要注意生命周期
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 危险的函数 - 会导致悬空引用（编译器会阻止）
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // 错误！返回对局部变量的引用
}  // s 离开作用域并被释放，引用就悬空了
*/

// 正确的方法 - 返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 返回所有权
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 借用的实际应用
    
    // 1. 函数参数使用引用避免所有权转移
    fn analyze_string(s: &str) -> (usize, usize, usize) {
        let length = s.len();
        let word_count = s.split_whitespace().count();
        let char_count = s.chars().count();
        (length, word_count, char_count)
    }
    
    let text = String::from("Hello, 世界! How are you?");
    let (len, words, chars) = analyze_string(&text);
    
    println!("文本分析:");
    println!("  原文: {}", text);  // text 仍然有效
    println!("  字节长度: {}", len);
    println!("  单词数: {}", words);
    println!("  字符数: {}", chars);
    
    // 2. 修改数据而不获取所有权
    fn append_exclamation(s: &mut String) {
        s.push('!');
    }
    
    let mut message = String::from("Hello, Rust");
    append_exclamation(&mut message);
    println!("修改后的消息: {}", message);
    
    // 3. 处理向量而不移动所有权
    fn find_max(numbers: &[i32]) -> Option<&i32> {
        numbers.iter().max()
    }
    
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some(max) = find_max(&nums) {
        println!("数组 {:?} 中的最大值: {}", nums, max);  // nums 仍然有效
    }
    
    // 4. 字符串切片
    let sentence = String::from("Hello world from Rust");
    let hello = get_first_word(&sentence);
    println!("第一个单词: '{}'", hello);
    println!("完整句子: '{}'", sentence);  // sentence 仍然有效
    
    // 5. 避免不必要的克隆
    fn process_data(data: &Vec<String>) -> Vec<String> {
        data.iter()
            .filter(|s| s.len() > 3)
            .map(|s| s.to_uppercase())
            .collect()
    }
    
    let data = vec![
        "hi".to_string(),
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
    ];
    
    let processed = process_data(&data);  // 借用而不是移动
    println!("原始数据: {:?}", data);      // data 仍然有效
    println!("处理后数据: {:?}", processed);
}

/*
借用与引用 vs 其他语言：

C 语言：
char* ptr = &variable;   // 指针，可能悬空
*ptr = 'x';             // 解引用，不安全

Python：
# 所有变量都是引用
a = [1, 2, 3]
b = a                   # b 和 a 指向同一个对象
b.append(4)             # 修改会影响 a

TypeScript/JavaScript：
let obj = {x: 1};
let ref = obj;          // 引用
ref.x = 2;              // 修改原对象

Java/Kotlin：
String s = "hello";
String ref = s;         // 引用（对于对象）
// 原始类型是值传递

Rust 的借用系统：
✅ 编译时检查：防止悬空引用
✅ 内存安全：保证引用的有效性
✅ 并发安全：防止数据竞争
✅ 零运行时开销：引用就是指针

核心规则：
1. 任意时刻，要么只能有一个可变引用，要么只能有多个不可变引用
2. 引用必须总是有效的（不能悬空）
3. 借用不获取所有权
4. 借用检查器确保内存安全

借用的优势：
- 避免不必要的数据复制
- 保持原始数据的所有权
- 编译时保证内存安全
- 支持高效的数据共享
*/
