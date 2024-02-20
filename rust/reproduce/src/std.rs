use std::io;

pub fn main1() {
    // 获取用户输入
    println!("请输入一些文本:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("无法读取输入");

    // 处理用户输入
    let trimmed_text = input_text.trim();
    println!("你输入的文本是: {}", trimmed_text);

    // 输出一些内容
    println!("这是一个简单的 Rust 命令行程序！");
}
