use crate::{
    typedefs::{AssignmentStatement, Expression, TableElement},
    token::TokenType,
};

pub fn parse_expr_to_json(expr: &Expression, indent: usize) -> String {
    let indent_str = "    ".repeat(indent);
    let next_indent_str = "    ".repeat(indent + 1);

    match expr {
        Expression::Unit(unit) => {
            if unit.token_type == TokenType::String || unit.token_type == TokenType::Identifier {
                return format!("\"{}\"", unit.value);
            } else {
                return format!("{}", unit.value);
            }
        }
        Expression::Table(table) => {
            let mut is_array: Option<bool> = None;
            let mut table_strings: Vec<String> = Vec::new();

            for element in &table.elements {
                match element {
                    TableElement::ArrayElement(element) => {
                        if is_array == None {
                            is_array.get_or_insert(true);
                        } else if is_array == Some(false) {
                            panic!("JSONTransformerError: arrays cannot have keyed elements");
                        }
                        table_strings.push(format!("{}{}", next_indent_str, element.value));
                    }
                    TableElement::KeyValueElement(key_value_element) => {
                        if is_array == None {
                            is_array.get_or_insert(false);
                        } else if is_array == Some(true) {
                            panic!("JSONTransformerError: objects cannot have keyless elements");
                        }
                        let expr_json: String =
                            parse_expr_to_json(&key_value_element.value, indent + 1);
                        table_strings.push(format!(
                            "{}\"{}\": {}",
                            next_indent_str, key_value_element.key.value, expr_json,
                        ));
                    }
                }
            }

            let brackets: (char, char);

            if let Some(true) = is_array {
                brackets = ('[', ']');
            } else {
                brackets = ('{', '}');
            }

            return format!(
                "{}\n{}\n{}{}",
                brackets.0,
                table_strings.join(",\n"),
                indent_str,
                brackets.1
            );
        }
    }
}
