use pest::iterators::Pair;
use crate::parser::{ExpressionParser, Rule};

impl ExpressionParser {
    pub fn get_name_and_value(record: Pair<Rule>) -> (String, String) {
        if let Some(inner) = record.into_inner().next() {
            let rule = inner.as_rule();
            let rule_name = format!("{:?}", rule);

            return if rule_name == "OCT_INTEGER" {
                ("INTEGER_CONST".to_string(), i32::from_str_radix(inner.as_str().trim_start_matches('0'), 8).unwrap().to_string())
            } else if rule_name == "HEX_INTEGER" {
                ("INTEGER_CONST".to_string(), i32::from_str_radix(inner.as_str().trim_start_matches("0x"), 16).unwrap().to_string())
            } else if rule_name == "DEC_INTEGER" {
                ("INTEGER_CONST".to_string(), inner.as_str().to_string())
            } else if is_uppercase(&rule_name) {
                (rule_name, inner.as_str().to_string())
            } else {
                Self::get_name_and_value(inner)
            }
        }

        ("".to_string(), "".to_string())
    }
}

fn is_uppercase(str: &str) -> bool {
    str.chars().all(|c| c.is_uppercase() || c == '_' || c.is_numeric())
}