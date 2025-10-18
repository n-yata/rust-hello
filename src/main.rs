use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    while {
        buffer.clear();
        handle.read_line(&mut buffer).expect("Failed to read input") != 0
    } {
        let expression = buffer.trim();

        if expression.is_empty() {
            continue;
        }

        match evaluate_expression(expression) {
            Ok(result) => println!("{}", result),
            Err(message) => println!("Error: {}", message),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let mut parts = expression.split_whitespace();

    let left = parts
        .next()
        .ok_or_else(|| "式が不正です (左項がありません)".to_string())?;
    let operator = parts
        .next()
        .ok_or_else(|| "式が不正です (演算子がありません)".to_string())?;
    let right = parts
        .next()
        .ok_or_else(|| "式が不正です (右項がありません)".to_string())?;

    if parts.next().is_some() {
        return Err("式が長すぎます".to_string());
    }

    let left_value = left
        .parse::<f64>()
        .map_err(|_| "左項を数値に変換できません".to_string())?;
    let right_value = right
        .parse::<f64>()
        .map_err(|_| "右項を数値に変換できません".to_string())?;

    let result = match operator {
        "+" => left_value + right_value,
        "-" => left_value - right_value,
        "*" | "x" | "X" => left_value * right_value,
        "/" => {
            if right_value == 0.0 {
                return Err("0 で割ることはできません".to_string());
            }
            left_value / right_value
        }
        _ => return Err("対応していない演算子です".to_string()),
    };

    Ok(result)
}
