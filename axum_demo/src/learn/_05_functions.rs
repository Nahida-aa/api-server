// 05 - 函数
// 函数定义、参数、返回值、高阶函数

#[allow(dead_code)]
pub fn run() {
    println!("=== 05. 函数 ===");
    
    // 1. 基本函数调用
    greet();
    greet_with_name("Rust");
    
    // 2. 有返回值的函数
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    let product = multiply(4, 7);
    println!("4 * 7 = {}", product);
    
    // 3. 表达式 vs 语句
    let result = expression_function(10);
    println!("表达式函数结果: {}", result);
    
    // 4. 多个返回值（使用元组）
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
    
    // 5. 函数作为参数（高阶函数）
    let numbers = vec![1, 2, 3, 4, 5];
    
    let sum = apply_operation(&numbers, add_one);
    println!("所有数字加1后的和: {}", sum);
    
    let sum_squares = apply_operation(&numbers, square);
    println!("所有数字平方后的和: {}", sum_squares);
    
    // 6. 闭包作为参数
    let sum_doubled = apply_operation(&numbers, |x| x * 2);
    println!("所有数字乘2后的和: {}", sum_doubled);
    
    // 7. 递归函数
    let n = 5;
    let factorial = factorial(n);
    println!("{} 的阶乘是 {}", n, factorial);
    
    let fib = fibonacci(10);
    println!("斐波那契数列第10项: {}", fib);
}

// 1. 无参数无返回值函数
fn greet() {
    println!("Hello, World!");
}

// 2. 有参数无返回值函数
fn greet_with_name(name: &str) {
    println!("Hello, {}!", name);
}

// 3. 有参数有返回值函数
fn add(a: i32, b: i32) -> i32 {
    a + b  // 表达式，不需要 return 和分号
}

// 4. 显式使用 return
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // 也可以使用 return
}

// 5. 表达式函数 vs 语句函数
fn expression_function(x: i32) -> i32 {
    // 最后一行是表达式，作为返回值
    if x > 0 {
        x * 2
    } else {
        0
    }
}

// 6. 返回元组（多个返回值）
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// 7. 函数作为参数的辅助函数
fn add_one(x: i32) -> i32 {
    x + 1
}

fn square(x: i32) -> i32 {
    x * x
}

// 8. 高阶函数 - 接受函数作为参数
fn apply_operation<F>(numbers: &[i32], operation: F) -> i32 
where
    F: Fn(i32) -> i32,
{
    numbers.iter().map(|&x| operation(x)).sum()
}

// 9. 递归函数
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 10. 泛型函数
fn largest<T>(list: &[T]) -> &T 
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 实用函数示例
    
    // 1. 计算器功能
    fn calculate(operation: &str, a: f64, b: f64) -> Option<f64> {
        match operation {
            "+" => Some(a + b),
            "-" => Some(a - b),
            "*" => Some(a * b),
            "/" if b != 0.0 => Some(a / b),
            "/" => None, // 除零错误
            _ => None,   // 不支持的操作
        }
    }
    
    let operations = ["+", "-", "*", "/"];
    let (a, b) = (10.0, 3.0);
    
    for op in operations {
        match calculate(op, a, b) {
            Some(result) => println!("{} {} {} = {}", a, op, b, result),
            None => println!("{} {} {} = 错误", a, op, b),
        }
    }
    
    // 2. 数据处理管道
    fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
        numbers
            .into_iter()
            .filter(|&x| x > 0)      // 过滤正数
            .map(|x| x * x)          // 平方
            .collect()               // 收集结果
    }
    
    let input = vec![-2, -1, 0, 1, 2, 3, 4];
    let result = process_numbers(input);
    println!("处理后的数字: {:?}", result);
    
    // 3. 字符串处理
    fn format_name(first: &str, last: &str) -> String {
        format!("{}, {}", last, first)
    }
    
    fn shout(text: &str) -> String {
        text.to_uppercase() + "!"
    }
    
    let name = format_name("张", "三");
    let excited_name = shout(&name);
    println!("格式化姓名: {}", name);
    println!("大声喊出: {}", excited_name);
    
    // 4. 配置验证
    fn validate_config(host: &str, port: u16) -> Result<String, String> {
        if host.is_empty() {
            return Err("主机名不能为空".to_string());
        }
        
        if port == 0 {
            return Err("端口号不能为0".to_string());
        }
        
        Ok(format!("{}:{}", host, port))
    }
    
    match validate_config("localhost", 8080) {
        Ok(address) => println!("有效地址: {}", address),
        Err(error) => println!("配置错误: {}", error),
    }
}

/*
与其他语言对比：

C 语言：
int add(int a, int b) {    // 必须指定返回类型
    return a + b;          // 必须使用 return
}

Python：
def add(a, b):             # 动态类型
    return a + b           # 必须使用 return

def greet(name="World"):   # 支持默认参数
    print(f"Hello, {name}!")

TypeScript：
function add(a: number, b: number): number {
    return a + b;          // 必须使用 return
}

const add = (a: number, b: number) => a + b;  // 箭头函数

Kotlin：
fun add(a: Int, b: Int): Int {
    return a + b
}

fun add(a: Int, b: Int) = a + b  // 单表达式函数

Rust 特色：
1. 最后一个表达式自动作为返回值（无需 return）
2. 语句vs表达式的明确区分
3. 模式匹配可用于函数参数
4. 强大的类型推断
5. 零成本抽象的函数指针和闭包
6. 所有权系统影响参数传递
*/
