// 41 - 结构体
// 自定义数据类型，类似其他语言的 class 或 struct

#[allow(dead_code)]
pub fn run() {
    println!("=== 41. 结构体 ===");
    
    // 1. 基本结构体使用
    basic_struct_usage();
    
    // 2. 结构体方法
    struct_methods();
    
    // 3. 关联函数
    associated_functions();
    
    // 4. 结构体更新语法
    struct_update_syntax();
    
    // 5. 元组结构体
    tuple_structs();
    
    // 6. 单元结构体
    unit_structs();
}

// 1. 基本结构体定义
#[derive(Debug)]  // 自动实现 Debug trait，用于打印
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 2. 带有方法的结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为 Rectangle 实现方法
impl Rectangle {
    // 方法 - 第一个参数是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 可变方法 - 第一个参数是 &mut self
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // 消费方法 - 第一个参数是 self
    fn into_square(self) -> Square {
        let side = std::cmp::min(self.width, self.height);
        Square { side }
    }
}

// 关联函数（类似静态方法）
impl Rectangle {
    // 关联函数 - 没有 self 参数
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
struct Square {
    side: u32,
}

fn basic_struct_usage() {
    println!("--- 基本结构体使用 ---");
    
    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("用户信息: {:?}", user1);
    
    // 访问字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    
    // 修改字段（需要可变绑定）
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("newemail@example.com");
    println!("修改后的邮箱: {}", user2.email);
}

fn struct_methods() {
    println!("--- 结构体方法 ---");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形: {:?}", rect1);
    println!("面积: {}", rect1.area());
    println!("周长: {}", rect1.perimeter());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3 吗？{}", rect1.can_hold(&rect3));
    
    // 可变方法
    let mut rect4 = Rectangle {
        width: 10,
        height: 20,
    };
    
    println!("缩放前: {:?}", rect4);
    rect4.scale(2);
    println!("缩放后: {:?}", rect4);
}

fn associated_functions() {
    println!("--- 关联函数 ---");
    
    // 使用关联函数创建实例
    let rect1 = Rectangle::new(20, 30);
    println!("使用 new 创建: {:?}", rect1);
    
    let rect2 = Rectangle::square(25);
    println!("使用 square 创建: {:?}", rect2);
    
    // 转换为正方形（消费原矩形）
    let square = rect2.into_square();
    println!("转换后的正方形: {:?}", square);
    // rect2 在这里已经不可用了
}

fn struct_update_syntax() {
    println!("--- 结构体更新语法 ---");
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 使用结构体更新语法创建新实例
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // 其余字段从 user1 复制
    };
    
    println!("新用户: {:?}", user2);
    // 注意：user1.username 和 user1.email 被移动了，user1 不再完全可用
    // 但是 user1.active 和 user1.sign_in_count 可以使用（它们实现了 Copy）
    println!("原用户的活跃状态: {}", user1.active);
}

// 3. 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn tuple_structs() {
    println!("--- 元组结构体 ---");
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("颜色: {:?}", black);
    println!("点: {:?}", origin);
    
    // 访问元组结构体的字段
    println!("红色分量: {}", black.0);
    println!("X 坐标: {}", origin.0);
    
    // 解构元组结构体
    let Color(r, g, b) = black;
    println!("RGB 值: ({}, {}, {})", r, g, b);
}

// 4. 单元结构体
#[derive(Debug)]
struct UnitStruct;

fn unit_structs() {
    println!("--- 单元结构体 ---");
    
    let unit = UnitStruct;
    println!("单元结构体: {:?}", unit);
    
    // 单元结构体通常用于实现 trait 而不需要存储数据
}

// 实际应用示例结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,  // 可选字段
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            email: None,
        }
    }
    
    fn with_email(mut self, email: String) -> Person {
        self.email = Some(email);
        self
    }
    
    fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
    }
    
    fn has_email(&self) -> bool {
        self.email.is_some()
    }
}

// 可复制到 main.rs 中运行的示例
pub fn main_example() {
    // 实际应用示例
    
    // 1. 配置结构体
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        debug: bool,
        max_connections: usize,
    }
    
    impl Config {
        fn new() -> Config {
            Config {
                host: "localhost".to_string(),
                port: 8080,
                debug: false,
                max_connections: 100,
            }
        }
        
        fn with_port(mut self, port: u16) -> Config {
            self.port = port;
            self
        }
        
        fn with_debug(mut self, debug: bool) -> Config {
            self.debug = debug;
            self
        }
        
        fn address(&self) -> String {
            format!("{}:{}", self.host, self.port)
        }
    }
    
    let config = Config::new()
        .with_port(3000)
        .with_debug(true);
    
    println!("服务器配置: {:?}", config);
    println!("监听地址: {}", config.address());
    
    // 2. 用户管理
    let person1 = Person::new("张三".to_string(), 25);
    let person2 = Person::new("李四".to_string(), 30)
        .with_email("lisi@example.com".to_string());
    
    println!("{}", person1.greet());
    println!("有邮箱吗？{}", person1.has_email());
    
    println!("{}", person2.greet());
    println!("有邮箱吗？{}", person2.has_email());
    
    // 3. 几何计算
    let rect = Rectangle::new(10, 20);
    let square = Rectangle::square(15);
    
    println!("矩形面积: {}", rect.area());
    println!("正方形面积: {}", square.area());
    println!("矩形能容纳正方形吗？{}", rect.can_hold(&square));
    
    // 4. 数据建模
    #[derive(Debug)]
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }
    
    impl Book {
        fn new(title: &str, author: &str, pages: u32) -> Book {
            Book {
                title: title.to_string(),
                author: author.to_string(),
                pages,
                available: true,
            }
        }
        
        fn borrow_book(&mut self) -> Result<(), &str> {
            if self.available {
                self.available = false;
                Ok(())
            } else {
                Err("书籍已被借出")
            }
        }
        
        fn return_book(&mut self) {
            self.available = true;
        }
    }
    
    let mut book = Book::new("Rust 程序设计语言", "Steve Klabnik", 500);
    println!("图书信息: {:?}", book);
    
    match book.borrow_book() {
        Ok(()) => println!("成功借出《{}》", book.title),
        Err(e) => println!("借书失败: {}", e),
    }
    
    book.return_book();
    println!("《{}》已归还", book.title);
}

/*
结构体 vs 其他语言：

C 语言：
struct Point {
    int x;
    int y;
};
// 只有数据，没有方法

Python：
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        return f"Hello, I'm {self.name}"

TypeScript：
interface Person {
    name: string;
    age: number;
}

class PersonClass {
    constructor(public name: string, public age: number) {}
    greet(): string {
        return `Hello, I'm ${this.name}`;
    }
}

Java/Kotlin：
data class Person(val name: String, val age: Int) {
    fun greet() = "Hello, I'm $name"
}

Rust 结构体特点：
✅ 所有权系统集成
✅ 零运行时开销
✅ 强类型安全
✅ 模式匹配支持
✅ trait 系统集成
✅ 自动内存管理

核心概念：
1. 结构体定义数据的结构
2. impl 块定义方法和关联函数
3. &self, &mut self, self 三种 self 类型
4. 结构体更新语法简化初始化
5. 元组结构体和单元结构体的特殊用途
*/
