#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut s: Vec<i32> = vec![];

    for x in inputs.iter() {
        if let Value(num) = x {
            s.push(*num);
            continue;
        }

        if s.len() < 2 {
            // 不够2个数字
            return None;
        }

        let b = s.pop().unwrap();
        let a = s.pop().unwrap();
        let cur = match x {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
            _ => panic!(),
        };
        s.push(cur);
    }
    if s.len() != 1 {
        None
    } else {
        Some(s[0])
    }
}
