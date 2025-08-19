// 03 - 基本操作符
// 算术、比较、逻辑操作符及其特殊性

#[allow(dead_code)]
pub fn run() {
    println!("=== 03. 基本操作符 ===");
    
    // 1. 算术操作符
    let a = 10;
    let b = 3;
    
    println!("算术操作 ({}, {}):", a, b);
    println!("  加法: {} + {} = {}", a, b, a + b);
    println!("  减法: {} - {} = {}", a, b, a - b);
    println!("  乘法: {} * {} = {}", a, b, a * b);
    println!("  除法: {} / {} = {}", a, b, a / b);      // 整数除法
    println!("  求余: {} % {} = {}", a, b, a % b);
    
    // 2. 浮点运算
    let x = 10.0;
    let y = 3.0;
    println!("浮点运算 ({}, {}):", x, y);
    println!("  除法: {} / {} = {}", x, y, x / y);      // 浮点除法
    
    // 3. 溢出处理 - Rust 的安全特性
    let max_u8 = 255u8;
    println!("u8 最大值: {}", max_u8);
    
    // 溢出检查方法
    match max_u8.checked_add(1) {
        Some(result) => println!("checked_add 结果: {}", result),
        None => println!("checked_add: 溢出！"),
    }
    
    // 饱和运算
    let saturated = max_u8.saturating_add(10);
    println!("saturating_add: {}", saturated);
    
    // 环绕运算
    let wrapped = max_u8.wrapping_add(1);
    println!("wrapping_add: {}", wrapped);
    
    // 4. 比较操作符
    let num1 = 5;
    let num2 = 10;
    
    println!("比较操作 ({}, {}):", num1, num2);
    println!("  相等: {} == {} = {}", num1, num2, num1 == num2);
    println!("  不等: {} != {} = {}", num1, num2, num1 != num2);
    println!("  小于: {} < {} = {}", num1, num2, num1 < num2);
    println!("  大于: {} > {} = {}", num1, num2, num1 > num2);
    println!("  小于等于: {} <= {} = {}", num1, num2, num1 <= num2);
    println!("  大于等于: {} >= {} = {}", num1, num2, num1 >= num2);
    
    // 5. 逻辑操作符
    let t = true;
    let f = false;
    
    println!("逻辑操作 ({}, {}):", t, f);
    println!("  逻辑与: {} && {} = {}", t, f, t && f);
    println!("  逻辑或: {} || {} = {}", t, f, t || f);
    println!("  逻辑非: !{} = {}", t, !t);
    
    // 6. 位操作符
    let bits1 = 0b1100u8;  // 12
    let bits2 = 0b1010u8;  // 10
    
    println!("位操作 ({:04b}, {:04b}):", bits1, bits2);
    println!("  位与: {:04b} & {:04b} = {:04b}", bits1, bits2, bits1 & bits2);
    println!("  位或: {:04b} | {:04b} = {:04b}", bits1, bits2, bits1 | bits2);
    println!("  位异或: {:04b} ^ {:04b} = {:04b}", bits1, bits2, bits1 ^ bits2);
    println!("  位非: !{:04b} = {:04b}", bits1, !bits1);
    println!("  左移: {:04b} << 1 = {:04b}", bits1, bits1 << 1);
    println!("  右移: {:04b} >> 1 = {:04b}", bits1, bits1 >> 1);
    
    // 7. 赋值操作符
    let mut value = 10;
    println!("赋值操作，初始值: {}", value);
    
    value += 5;
    println!("  += 5: {}", value);
    
    value -= 3;
    println!("  -= 3: {}", value);
    
    value *= 2;
    println!("  *= 2: {}", value);
    
    value /= 4;
    println!("  /= 4: {}", value);
    
    value %= 5;
    println!("  %= 5: {}", value);
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 安全的数学运算
    let a = 100u8;
    let b = 200u8;
    
    // 使用 checked 系列方法避免溢出
    match a.checked_add(b) {
        Some(sum) => println!("安全加法: {} + {} = {}", a, b, sum),
        None => println!("加法溢出: {} + {} 超出范围", a, b),
    }
    
    // 实用的位操作示例
    let flags = 0b0000_1010u8;  // 二进制标志位
    
    println!("标志位: {:08b}", flags);
    println!("第1位是否设置: {}", (flags & 0b0000_0010) != 0);
    println!("第3位是否设置: {}", (flags & 0b0000_1000) != 0);
    
    // 设置第0位
    let new_flags = flags | 0b0000_0001;
    println!("设置第0位后: {:08b}", new_flags);
    
    // 清除第3位
    let cleared_flags = new_flags & !0b0000_1000;
    println!("清除第3位后: {:08b}", cleared_flags);
}

/*
与其他语言对比：

C 语言：
- 整数溢出是未定义行为
- 位操作语法相同
- 没有溢出检查

Python：
- 整数可以任意大小
- // 用于整数除法
- ** 用于幂运算
- 位操作使用相同符号

TypeScript/JavaScript：
- 只有 number 类型（双精度浮点）
- === 和 !== 用于严格比较
- ** 用于幂运算

Kotlin：
- 类似 Java，有溢出
- === 用于引用比较
- shl, shr 用于位移

Rust 特色：
1. 默认情况下 debug 模式会 panic，release 模式会环绕
2. 提供 checked_*, saturating_*, wrapping_* 方法
3. 严格的类型系统，不同类型不能直接运算
4. 没有隐式类型转换
*/
