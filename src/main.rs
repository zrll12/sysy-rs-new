mod parser;

use std::{env, fs};

use pest_derive::Parser;
use crate::parser::ExpressionParser;

// #[derive(Parser)]
// #[grammar = "lexer.pest"] // 指向你的 .pest 文件
struct SysYLexer;

fn main() {
    // 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了文件名
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // 获取文件名
    let filename = &args[1];

    // 读取输入文件
    let mut input = fs::read_to_string(filename).expect("Failed to read file");
    if !input.ends_with('\n') {
        input.push_str("\n");
    }

    // 词法分析
    let tokens = ExpressionParser::tokenize_print(&input);
}