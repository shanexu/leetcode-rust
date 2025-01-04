fn is_operator(c: char) -> bool {
    c != '(' && c != ')'
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' | '~' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn infix_to_rpn(input: &str) -> String {
    let mut output = String::new();
    let mut stack = Vec::new();
    let mut num_buffer = String::new();
    let mut is_last_char_open = true;

    for mut c in input.chars() {
        if c.is_whitespace() {
            continue; // Skip whitespace
        } else if c.is_digit(10) {
            num_buffer.push(c);
            is_last_char_open = false;
        } else {
            if !num_buffer.is_empty() {
                output.push_str(&num_buffer);
                output.push(' ');
                num_buffer.clear();
            }

            if c == '(' {
                stack.push(c);
                is_last_char_open = true;
            } else if c == ')' {
                is_last_char_open = false;
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    } else {
                        output.push(top);
                        output.push(' ');
                    }
                }
            }  else if is_operator(c) {
                if is_last_char_open && c == '-' {
                    c = '~';
                }
                is_last_char_open = false;
                while let Some(&top) = stack.last() {
                    if is_operator(top) && precedence(top) >= precedence(c) {
                        output.push(stack.pop().unwrap());
                        output.push(' ');
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
        }
    }

    if !num_buffer.is_empty() {
        output.push_str(&num_buffer);
        output.push(' ');
    }

    while let Some(op) = stack.pop() {
        output.push(op);
        output.push(' ');
    }

    output.trim().to_string()
}

fn main() {
    let input = "-20 - (30 + 42) * (-22) / 55";
    let rpn = infix_to_rpn(input);
    println!("Infix expression: {}", input);
    println!("RPN expression: {}", rpn);
}
