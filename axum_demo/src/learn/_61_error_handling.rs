// 61 - 错误处理
// Rust 的错误处理哲学：显式、类型安全、零成本

#[allow(dead_code)]
pub fn run() {
    println!("=== 61. 错误处理 ===");
    
    // 1. panic! 和不可恢复错误
    unrecoverable_errors();
    
    // 2. Result 和可恢复错误
    recoverable_errors();
    
    // 3. ? 操作符
    question_mark_operator();
    
    // 4. 自定义错误类型
    custom_errors();
    
    // 5. 错误传播
    error_propagation();
    
    // 6. 多种错误处理模式
    error_handling_patterns();
}

fn unrecoverable_errors() {
    println!("--- 不可恢复错误 (panic!) ---");
    
    // panic! 宏用于不可恢复的错误
    // 注意：这些示例被注释掉了，因为 panic! 会终止程序
    
    println!("演示 panic! 的情况（已注释掉实际调用）:");
    println!("1. panic!(\"明确的错误消息\");");
    println!("2. 数组越界访问");
    println!("3. unwrap() 在 None 或 Err 上");
    println!("4. expect() 在 None 或 Err 上");
    
    // 安全的边界检查
    let v = vec![1, 2, 3];
    
    // 不安全：可能 panic
    // let element = v[10];  // 会 panic
    
    // 安全：返回 Option
    match v.get(10) {
        Some(element) => println!("元素: {}", element),
        None => println!("索引超出范围"),
    }
    
    // unwrap 的安全替代
    let some_value = Some(5);
    let value = some_value.unwrap_or(0);  // 提供默认值
    println!("安全获取值: {}", value);
}

fn recoverable_errors() {
    println!("--- 可恢复错误 (Result) ---");
    
    // 文件操作示例
    use std::fs::File;
    use std::io::ErrorKind;
    
    let filename = "nonexistent.txt";
    
    match File::open(filename) {
        Ok(file) => println!("成功打开文件: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("文件 {} 不存在", filename);
                // 可以尝试创建文件
                match File::create(filename) {
                    Ok(_) => println!("创建文件成功"),
                    Err(e) => println!("创建文件失败: {}", e),
                }
            },
            other_error => {
                println!("打开文件时出现其他错误: {:?}", other_error);
            }
        },
    }
    
    // 使用 unwrap_or_else 简化错误处理
    let _file = File::open("another_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("another_file.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });
}

fn question_mark_operator() {
    println!("--- ? 操作符 ---");
    
    // ? 操作符简化错误传播
    fn read_username_from_file(filename: &str) -> Result<String, std::io::Error> {
        use std::fs::File;
        use std::io::Read;
        
        let mut file = File::open(filename)?;  // 如果出错，立即返回错误
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;   // 如果出错，立即返回错误
        Ok(contents)
    }
    
    // 更简洁的版本
    fn read_username_from_file_short(filename: &str) -> Result<String, std::io::Error> {
        use std::fs;
        fs::read_to_string(filename)  // 一行搞定
    }
    
    match read_username_from_file("username.txt") {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取用户名失败: {}", e),
    }
    
    match read_username_from_file_short("username.txt") {
        Ok(username) => println!("用户名（简洁版本）: {}", username),
        Err(e) => println!("读取用户名失败（简洁版本）: {}", e),
    }
    
    // ? 操作符也可以用于 Option
    fn get_first_char(text: &str) -> Option<char> {
        text.chars().next()
    }
    
    fn get_first_uppercase_char(text: &str) -> Option<char> {
        let first_char = get_first_char(text)?;  // 如果是 None，立即返回 None
        if first_char.is_uppercase() {
            Some(first_char)
        } else {
            None
        }
    }
    
    println!("第一个大写字符: {:?}", get_first_uppercase_char("Hello"));
    println!("第一个大写字符: {:?}", get_first_uppercase_char("hello"));
    println!("第一个大写字符: {:?}", get_first_uppercase_char(""));
}

// 自定义错误类型
#[derive(Debug)]
enum CalculatorError {
    DivisionByZero,
    InvalidInput(String),
    Overflow,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "除数不能为零"),
            CalculatorError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
            CalculatorError::Overflow => write!(f, "计算结果溢出"),
        }
    }
}

impl std::error::Error for CalculatorError {}

fn custom_errors() {
    println!("--- 自定义错误类型 ---");
    
    fn safe_divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else if a.is_infinite() || b.is_infinite() {
            Err(CalculatorError::InvalidInput("无穷大".to_string()))
        } else {
            let result = a / b;
            if result.is_infinite() {
                Err(CalculatorError::Overflow)
            } else {
                Ok(result)
            }
        }
    }
    
    let calculations = [(10.0, 2.0), (10.0, 0.0), (f64::INFINITY, 2.0)];
    
    for (a, b) in calculations {
        match safe_divide(a, b) {
            Ok(result) => println!("{} ÷ {} = {}", a, b, result),
            Err(e) => println!("{} ÷ {} 错误: {}", a, b, e),
        }
    }
}

fn error_propagation() {
    println!("--- 错误传播 ---");
    
    // 使用 ? 操作符传播错误
    fn process_data(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("输入不能为空".into());
        }
        
        let number: i32 = trimmed.parse()?;  // 解析错误会自动传播
        
        if number < 0 {
            return Err("数字不能为负数".into());
        }
        
        Ok(number * 2)
    }
    
    let inputs = ["  42  ", "", "abc", "-5", "21"];
    
    for input in inputs {
        match process_data(input) {
            Ok(result) => println!("输入 '{}' 处理结果: {}", input, result),
            Err(e) => println!("输入 '{}' 处理失败: {}", input, e),
        }
    }
}

fn error_handling_patterns() {
    println!("--- 错误处理模式 ---");
    
    // 1. 链式错误处理
    fn chain_operations(x: i32) -> Result<i32, &'static str> {
        Ok(x)
            .and_then(|x| if x > 0 { Ok(x) } else { Err("值必须为正数") })
            .and_then(|x| if x < 100 { Ok(x) } else { Err("值不能大于等于100") })
            .map(|x| x * 2)
    }
    
    for value in [5, -3, 150, 50] {
        match chain_operations(value) {
            Ok(result) => println!("链式操作 {} -> {}", value, result),
            Err(e) => println!("链式操作 {} 失败: {}", value, e),
        }
    }
    
    // 2. 收集错误
    fn validate_numbers(numbers: &[&str]) -> Result<Vec<i32>, Vec<String>> {
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        for num_str in numbers {
            match num_str.parse::<i32>() {
                Ok(num) => results.push(num),
                Err(e) => errors.push(format!("'{}': {}", num_str, e)),
            }
        }
        
        if errors.is_empty() {
            Ok(results)
        } else {
            Err(errors)
        }
    }
    
    let inputs = ["1", "2", "abc", "4", "def"];
    match validate_numbers(&inputs) {
        Ok(numbers) => println!("所有数字: {:?}", numbers),
        Err(errors) => println!("解析错误: {:?}", errors),
    }
    
    // 3. 早期返回模式
    fn complex_calculation(a: i32, b: i32, c: i32) -> Result<i32, &'static str> {
        if a == 0 { return Err("a 不能为 0"); }
        if b == 0 { return Err("b 不能为 0"); }
        if c == 0 { return Err("c 不能为 0"); }
        
        let step1 = a * b;
        let step2 = step1 / c;
        let step3 = step2 + 10;
        
        Ok(step3)
    }
    
    println!("复杂计算 (2, 3, 1): {:?}", complex_calculation(2, 3, 1));
    println!("复杂计算 (0, 3, 1): {:?}", complex_calculation(0, 3, 1));
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 实际应用的错误处理
    
    // 1. 配置解析器
    #[derive(Debug)]
    enum ConfigError {
        MissingKey(String),
        InvalidValue { key: String, value: String },
        ParseError(String),
    }
    
    impl std::fmt::Display for ConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ConfigError::MissingKey(key) => write!(f, "缺少配置项: {}", key),
                ConfigError::InvalidValue { key, value } => 
                    write!(f, "配置项 {} 的值 {} 无效", key, value),
                ConfigError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            }
        }
    }
    
    impl std::error::Error for ConfigError {}
    
    fn parse_config_line(line: &str) -> Result<(String, String), ConfigError> {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(ConfigError::ParseError(
                format!("无效的配置行: {}", line)
            ));
        }
        
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        
        if key.is_empty() {
            return Err(ConfigError::ParseError("键不能为空".to_string()));
        }
        
        Ok((key, value))
    }
    
    fn validate_config_value(key: &str, value: &str) -> Result<(), ConfigError> {
        match key {
            "port" => {
                value.parse::<u16>().map_err(|_| ConfigError::InvalidValue {
                    key: key.to_string(),
                    value: value.to_string(),
                })?;
            },
            "debug" => {
                value.parse::<bool>().map_err(|_| ConfigError::InvalidValue {
                    key: key.to_string(),
                    value: value.to_string(),
                })?;
            },
            _ => {} // 其他配置项暂时接受任何值
        }
        Ok(())
    }
    
    let config_lines = [
        "host=localhost",
        "port=8080",
        "debug=true",
        "invalid_line",
        "port=invalid_port",
        "=empty_key",
    ];
    
    for line in config_lines {
        match parse_config_line(line) {
            Ok((key, value)) => {
                match validate_config_value(&key, &value) {
                    Ok(()) => println!("✓ 配置: {} = {}", key, value),
                    Err(e) => println!("✗ 验证失败: {}", e),
                }
            },
            Err(e) => println!("✗ 解析失败: {}", e),
        }
    }
    
    // 2. 网络请求错误处理（模拟）
    #[derive(Debug)]
    enum NetworkError {
        Timeout,
        ConnectionRefused,
        InvalidUrl(String),
        ServerError(u16),
    }
    
    impl std::fmt::Display for NetworkError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                NetworkError::Timeout => write!(f, "请求超时"),
                NetworkError::ConnectionRefused => write!(f, "连接被拒绝"),
                NetworkError::InvalidUrl(url) => write!(f, "无效的URL: {}", url),
                NetworkError::ServerError(code) => write!(f, "服务器错误: {}", code),
            }
        }
    }
    
    fn simulate_request(url: &str) -> Result<String, NetworkError> {
        if url.is_empty() {
            return Err(NetworkError::InvalidUrl("URL不能为空".to_string()));
        }
        
        if !url.starts_with("http") {
            return Err(NetworkError::InvalidUrl(url.to_string()));
        }
        
        // 模拟不同的错误情况
        match url {
            "http://timeout.com" => Err(NetworkError::Timeout),
            "http://refused.com" => Err(NetworkError::ConnectionRefused),
            "http://server-error.com" => Err(NetworkError::ServerError(500)),
            _ => Ok(format!("成功响应来自: {}", url)),
        }
    }
    
    let urls = [
        "http://example.com",
        "",
        "invalid-url",
        "http://timeout.com",
        "http://refused.com",
        "http://server-error.com",
    ];
    
    for url in urls {
        match simulate_request(url) {
            Ok(response) => println!("✓ {}", response),
            Err(e) => println!("✗ 请求 '{}' 失败: {}", url, e),
        }
    }
    
    println!("\n错误处理总结:");
    println!("1. 使用 Result<T, E> 处理可恢复错误");
    println!("2. 使用 ? 操作符简化错误传播");
    println!("3. 创建自定义错误类型提供更好的错误信息");
    println!("4. panic! 只用于真正不可恢复的错误");
    println!("5. 使用 Box<dyn Error> 处理多种错误类型");
}

/*
错误处理 vs 其他语言：

C 语言：
- 使用返回码或全局 errno
- 容易忽略错误检查
- 没有类型安全

Java/Kotlin：
- 检查异常强制处理
- 运行时异常可能被忽略
- try-catch 有性能开销

Python：
- 异常机制，容易被忽略
- duck typing 导致运行时错误

TypeScript/JavaScript：
- Error 对象和 try-catch
- Promise.catch() 或 async/await
- 容易忽略错误处理

Rust 的优势：
✅ 编译时强制错误处理
✅ 类型安全的错误信息
✅ 零运行时开销
✅ 明确区分可恢复和不可恢复错误
✅ ? 操作符简化错误传播
✅ Result<T, E> 和 Option<T> 的完美集成

最佳实践：
1. 优先使用 Result 而不是 panic!
2. 使用 ? 操作符简化错误传播
3. 创建有意义的自定义错误类型
4. 提供清晰的错误消息
5. 考虑错误的调用者如何处理
*/
