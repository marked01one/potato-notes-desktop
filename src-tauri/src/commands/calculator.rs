pub fn calculator(query: &str) -> i32 {
    for i in 0..query.len() {
        if let Some(c) = query.chars().nth(i) {
            println!("{}", c)
        }

    }

    return -1
}

fn infixToPostfix(query: &str) -> Vec<char> {
    let output: Vec<char> = Vec::new();

    return output
}