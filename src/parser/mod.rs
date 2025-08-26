mod names;

use std::process::exit;
use pest::error::LineColLocation::Pos;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lexer.pest"]
pub struct ExpressionParser {}

pub enum Token {

}

impl ExpressionParser {
    pub fn tokenize(input: &str) {
        let pair = Self::parse(Rule::file, input).unwrap();
        // println!("{:?}", pair);
    }

    pub fn tokenize_print(input: &str) {
        let pair = Self::parse(Rule::file, input).unwrap_or_else(|err| {
            if let Pos((col, _)) = err.line_col {
                eprintln!("Error type A at Line {col}: Mysterious character ({}).", err.line());
            }
            exit(0);
        }).next().unwrap();

        for record in pair.clone().into_inner() {
            let (name, value) = Self::get_name_and_value(record.clone());
            if name.is_empty() {
                continue;
            }
            eprintln!("{} {} at line {}", name, value, record.line_col().0)
            
            // if let Some(inner) = record.into_inner().next() {
            //     eprintln!("{:?} {} at line {}", inner.as_rule(), inner.as_str(), inner.line_col().0)
            //     // match inner.as_rule() {
            //     //     Rule::INT => eprintln!("INT {}", a),
            //     //     _ => {  },
            //     // }
            // }
        }
    }
}