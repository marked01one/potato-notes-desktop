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
        stack.push(0.1);
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