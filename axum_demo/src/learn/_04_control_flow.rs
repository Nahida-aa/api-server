// 04 - 控制流
// if/else、循环、模式匹配

#[allow(dead_code)]
pub fn run() {
    println!("=== 04. 控制流 ===");
    
    // 1. if 表达式 - 注意：Rust 中 if 是表达式，可以返回值
    let number = 6;
    
    // 基本 if/else
    if number % 4 == 0 {
        println!("{} 能被 4 整除", number);
    } else if number % 3 == 0 {
        println!("{} 能被 3 整除", number);
    } else if number % 2 == 0 {
        println!("{} 能被 2 整除", number);
    } else {
        println!("{} 不能被 4、3、2 整除", number);
    }
    
    // if 作为表达式使用（类似三元操作符）
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("条件表达式结果: {}", number);
    
    // 2. loop 循环 - 无限循环
    println!("loop 循环示例:");
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // loop 可以返回值
        }
        
        print!("{} ", counter);
    };
    
    println!("\nloop 返回值: {}", result);
    
    // 3. while 循环
    println!("while 循环示例:");
    let mut number = 3;
    
    while number != 0 {
        print!("{}! ", number);
        number -= 1;
    }
    println!("发射! 🚀");
    
    // 4. for 循环 - 遍历迭代器
    println!("for 循环示例:");
    
    // 遍历数组
    let array = [10, 20, 30, 40, 50];
    for element in array {
        print!("{} ", element);
    }
    println!();
    
    // 遍历范围
    for i in 1..4 {  // 1, 2, 3 (不包含 4)
        print!("范围 {} ", i);
    }
    println!();
    
    for i in 1..=4 { // 1, 2, 3, 4 (包含 4)
        print!("包含范围 {} ", i);
    }
    println!();
    
    // 带索引遍历
    for (index, value) in array.iter().enumerate() {
        println!("  索引 {}: 值 {}", index, value);
    }
    
    // 5. 循环标签和 break/continue
    println!("嵌套循环与标签:");
    
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("  在 ({}, {}) 处跳出外层循环", i, j);
                break 'outer;
            }
            print!("({}, {}) ", i, j);
        }
        println!();
    }
    
    // 6. match 表达式 - 强大的模式匹配
    println!("match 表达式示例:");
    
    let number = 13;
    
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("这是一个小质数"),
        13..=19 => println!("十几的数字"),
        _ => println!("其他数字"),
    }
    
    // match 返回值
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("布尔值 {} 对应二进制 {}", boolean, binary);
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 实用的控制流示例
    
    // 1. 猜数字游戏的简化版本
    let secret_number = 42;
    let guesses = [35, 50, 42, 30];
    
    for (attempt, guess) in guesses.iter().enumerate() {
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("第 {} 次猜测 {} 太小了!", attempt + 1, guess),
            std::cmp::Ordering::Greater => println!("第 {} 次猜测 {} 太大了!", attempt + 1, guess),
            std::cmp::Ordering::Equal => {
                println!("第 {} 次猜测 {} 正确! 🎉", attempt + 1, guess);
                break;
            }
        }
    }
    
    // 2. 计算阶乘
    let n = 5;
    let mut factorial = 1;
    let mut i = 1;
    
    while i <= n {
        factorial *= i;
        i += 1;
    }
    
    println!("{} 的阶乘是 {}", n, factorial);
    
    // 3. 过滤和处理数据
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut evens = Vec::new();
    
    for num in numbers {
        if num % 2 == 0 {
            evens.push(num * num); // 偶数的平方
        }
    }
    
    println!("偶数的平方: {:?}", evens);
    
    // 4. 状态机示例
    enum State {
        Start,
        Processing,
        Complete,
        Error,
    }
    
    let states = [State::Start, State::Processing, State::Complete];
    
    for state in states {
        let message = match state {
            State::Start => "开始处理",
            State::Processing => "正在处理...",
            State::Complete => "处理完成",
            State::Error => "处理出错",
        };
        println!("状态: {}", message);
    }
}

/*
与其他语言对比：

C 语言：
if (condition) { }       // 语句
for (int i = 0; i < 10; i++) {
    // print
    printf("%d ", i);
}
switch (value) { case 1: break; }

Python：
if condition:            # 语句
for i in range(10):      # 遍历可迭代对象
    print('i: {}'.format(i))
    print(f'i: {i}')  # f-string
# 没有 switch，使用 if/elif

TypeScript：
if (condition) { }       // 语句
for (let i = 0; i < 10; i++) {
    console.log('i: {}'.format(i));
    console.log(`i: ${i}`);
}
for (const item of array) { }
switch (value) { case 1: break; }

Kotlin：
if (condition) { }       // 表达式
for (i in 1..10) { 
    println(String.format("i: %d", i))
    println("i: $i")
}
// 范围
when (value) { 1 -> "one" }  // 类似 match

Java:
if (condition) { }       // 语句
for (int i = 0; i < 10; i++) {
    System.out.println(String.format("i: %d", i));
}

CSharp:
if (condition) { }       // 语句
for (int i = 0; i < 10; i++) {
    Console.WriteLine($"i: {i}");
}

Rust 特色：
1. if 是表达式，可以返回值
2. loop 可以返回值
3. match 是表达式，必须穷尽所有可能
4. 强大的模式匹配
5. 循环标签用于精确控制嵌套循环
6. 范围语法 1..10 和 1..=10
*/
