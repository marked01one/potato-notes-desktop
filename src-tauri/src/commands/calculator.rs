pub fn calculator(query: &str) -> f32 {
    let postfix = to_postfix(query);
    return evaluate_postfix(postfix);
}


fn to_postfix(query: &str) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    let mut output: Vec<char> = Vec::new();

    for c in query.chars() {
        match c {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => output.push(c),
            '(' => stack.push(c),
            ')' => {
                while !stack.is_empty() && stack[stack.len()-1] != '(' {
                    if let Some(popped) = stack.pop() {
                        output.push(popped);
                    }
                    else {
                        panic!("Null exception in the to_postfix() stack!")
                    }
                }
                stack.pop();
            },
            '^'|'/'|'*'|'+'|'-' => {
                while !stack.is_empty() && (pemdas(c) <= pemdas(stack[stack.len()-1]) && associatvity(c) == 'L') {
                    if let Some(popped) = stack.pop() {
                        output.push(popped);
                    }   
                    else {
                        panic!("Null exception in the to_postfix() stack!")
                    }
                }
                stack.push(c);
            },
            _ => continue
        }
    }

    while !stack.is_empty() {
        if let Some(popped) = stack.pop() {
            output.push(popped);
        }
        else {
            panic!("Null exception in the to_postfix() stack!")
        }
    }    

    return output;
}


fn evaluate_postfix(pf: Vec<char>) -> f32 {
    let mut stack: Vec<f32> = Vec::new();

    for c in pf {
        if !c.is_numeric() {
            let op2 = stack.pop().unwrap();
            let op1 = stack.pop().unwrap();

            match c {
                '+' => stack.push(op1+op2),
                '-' => stack.push(op1-op2),
                '*' => stack.push(op1*op2),
                '/' => stack.push(op1/op2),
                '^' => stack.push(f32::powf(op1, op2)),
                _ => continue
            }
        }
        else {
            let c_float = c.to_digit(10);
            if c_float.is_some() {
                stack.push(c_float.unwrap() as f32);
            }
            else {
                panic!("Null exception in the evaluate_postfix() stack!")
            }
        };
    }
    return stack[0];
}


fn pemdas(c: char) -> i32 {
    match c {
        '^'     => return 3,
        '/'|'*' => return 2,
        '+'|'-' => return 1,
        _       => return -1
    }
}


fn associatvity(c: char) -> char {
    if c == '^' { 
        return 'L';
    }
    else {
        return 'R';
    }
}