pub struct Expression {
    x: i32,
    y: i32,
    operator: Operator
}

pub enum Operator {
    Null,
    Plus,
    Minus,
    Divide,
    Multiply,
}

// expr = expression
pub fn parse(expr: &String) -> Expression {
    let mut num_buffer = String::new();

    let mut expression = Expression { x: 0, y: 0, operator: Operator::Null };

    for (i, chr) in expr.chars().enumerate() {
        if chr == ' ' { continue; } // If i is a space skip this iteration
        if chr.is_numeric() { // Check if the char is a numeric
            num_buffer.push(chr); // Append the char to num_buffer
        } else {
            match chr { // Set right op id
                '+' => expression.operator = Operator::Plus,
                '-' => expression.operator = Operator::Minus,
                '*' => expression.operator = Operator::Multiply,
                '/' => expression.operator = Operator::Divide,
                _ => panic!("Invalid operator ({})", chr)
            }
            if expression.x == 0 {
                expression.x = num_buffer.parse::<i32>().unwrap(); // Convert num_buffer to i32 and store it into x
                num_buffer.clear()
            }
        }
        if i == expr.len() - 1 { // if it's last iteration
            expression.y = num_buffer.parse::<i32>().unwrap(); // Convert num_buffer to i32 and store it into x
        }
    }
    return expression;
}

pub fn process(expression: Expression) -> f32 {
    let x = expression.x as f32;
    let y = expression.y as f32;

    match expression.operator {
        Operator::Plus     => x + y,
        Operator::Minus    => x - y,
        Operator::Multiply => x * y,
        Operator::Divide   => x / y,
        _ => panic!("Uknown operator")
    }
}