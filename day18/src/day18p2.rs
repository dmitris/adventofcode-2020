use std::error::Error;
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Token {
    Literal(i64),
    Plus,
    Mult,
    ParenLeft,
    ParenRight,
}

enum ParseState {
    None,
    InDigit,
 }

fn print_stack(stack: &Vec<Token>) {
    for (i, t) in stack.iter().enumerate() {
        println!("stack[{:?}]: {:?}", i, t);
    }
}

fn reduce(stack: &mut Vec<Token>) -> Result<(), Box<dyn Error>> {
    if stack.len() < 3 {
        return Ok(());
    }
    print_stack(stack);
    for i in 0..2 {
    loop {
        if stack.len() < 3 {
            return Ok(());
        }
        let c = stack[stack.len() - 1];
        let b = stack[stack.len() - 2];
        let a = stack[stack.len() - 3];
        match (a, b, c) {
            (Token::ParenLeft, Token::Literal(n), Token::ParenRight) => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Literal(n));
            }
            (Token::Literal(n), Token::Plus, Token::Literal(m)) => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Literal(n + m));
            }
            (Token::Literal(n), Token::Mult, Token::Literal(m)) => {
                if i == 1 {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Literal(n * m));
                }
            }
            _ => {
                // println!("irreducable sequence...");
                break;
            }
        };
    }
};
    Ok(())
}

fn eval(line: &str) -> Result<i64, Box<dyn Error>> {
    let mut stack: Vec<Token> = Vec::new();
    let mut acc: i64 = 0;
    let mut state: ParseState = ParseState::None;
    let mut after_plus = false;
    for b in line.as_bytes().iter() {
        match b {
            b'(' => stack.push(Token::ParenLeft),
            b')' => {
                match state {
                    ParseState::InDigit => {
                        stack.push(Token::Literal(acc));
                        acc = 0;
                        reduce(&mut stack)?;
                    }
                    _ => (),
                };
                state = ParseState::None;
                stack.push(Token::ParenRight);
                reduce(&mut stack)?;
            }
            b'+' => {
                stack.push(Token::Plus);
            }
            b'*' => stack.push(Token::Mult),
            b' ' | b'\t' => {
                match state {
                    ParseState::InDigit => {
                        stack.push(Token::Literal(acc));
                        acc = 0;
                        reduce(&mut stack)?;
                    }
                    _ => (),
                };
                state = ParseState::None;
            }
            b'0'..=b'9' => {
                acc = (10 * acc) + *b as i64 - 48; // ascii convertion to digit
                state = ParseState::InDigit;
            }
            _ => return Err("bad input character".into()),
        };
    }
    match state {
        ParseState::InDigit => stack.push(Token::Literal(acc)),
        _ => (),
    };
    reduce(&mut stack)?;
    if stack.len() != 1 {
        return Err("failure in eval - malformed input".into());
    }
    match stack[0] {
        Token::Literal(n) => Ok(n),
        _ => Err("bad input".into()),
    }
}

fn main() {
    let result: i64 = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| eval(&line))
        .filter_map(Result::ok)
        .sum();
    println!("sum: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(2, eval("(2)").unwrap());
        assert_eq!(3, eval("1 + 2").unwrap());
        assert_eq!(20, eval("2 * 3 + 7").unwrap());
        // assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        // assert_eq!(
        //     12240,
        //     eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap()
        // );
        // assert_eq!(
        //     13632,
        //     eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap()
        // );
    }
}
