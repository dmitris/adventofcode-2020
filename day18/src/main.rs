use std::error::Error;
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Token {
    Literal(i64),
    Plus,
    Mult,
}

fn find_closing_paren(inp: &[u8]) -> Result<usize, Box<dyn Error>> {
    let mut cnt: u16 = 0;
    for i in 0..inp.len() {
        match inp[i] {
            b'(' => cnt += 1,
            b')' => {
                cnt -= 1;
                if cnt == 0 {
                    return Ok(i);
                }
            }
            _ => (),
        }
    }
    Err("closing parens not found".into())
}

// return the index after the end of the number
fn index_past_number(inp: &[u8]) -> usize {
    for i in 1..inp.len() {
        match inp[i] {
            b'0'..=b'9' => continue,
            _ => return i,
        };
    }
    inp.len()
}

fn reduce(stack: &mut Vec<Token>) -> Result<(), Box<dyn Error>> {
    if stack.len() < 3 {
        return Err("stack len < 3".into());
    }
    loop {
        if stack.len() < 3 {
            return Ok(());
        }
        let c = stack[stack.len() - 1];
        let b = stack[stack.len() - 2];
        let a = stack[stack.len() - 3];
        match (a, b, c) {
            (Token::Literal(n), Token::Plus, Token::Literal(m)) => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Literal(n + m));
            }
            _ => return Ok(()),
        };
    } // loop
}

fn eval(inp: &[u8]) -> Result<i64, Box<dyn Error>> {
    let mut i: usize = 0;
    let mut stack: Vec<Token> = Vec::new();

    loop {
        if i >= inp.len() {
            break;
        }
        match inp[i] {
            b'(' => {
                let ind = find_closing_paren(&inp[i..])?;
                let val = eval(&inp[i + 1..(i + ind)])?;
                stack.push(Token::Literal(val));
                if stack.len() >= 3 {
                    match stack[stack.len() - 2] {
                        Token::Plus => reduce(&mut stack)?,
                        _ => (),
                    };
                }
                i += ind; // additional +1 is after match
            }
            b')' => return Err("invalid input".into()),
            b'+' => stack.push(Token::Plus),
            b'*' => stack.push(Token::Mult),
            b'0'..=b'9' => {
                let ind = index_past_number(&inp[i..]);
                let mut acc: i64 = 0;
                for j in i..i + ind {
                    acc = (10 * acc) + inp[j] as i64 - 48;
                }
                stack.push(Token::Literal(acc));
                if stack.len() >= 3 {
                    match stack[stack.len() - 2] {
                        Token::Plus => reduce(&mut stack)?,
                        _ => (),
                    };
                }
            }
            b' ' => (),
            _ => return Err("unknown character".into()),
        };
        i += 1;
    } // loop

    // we just need to multiply through whatever is left
    let mut acc = 1;
    for t in stack.iter() {
        match t {
            Token::Literal(n) => acc *= n,
            Token::Mult => (),
            _ => return Err("bad token at the end of eval - not number or '*'".into()),
        }
    }
    Ok(acc)
}

fn main() {
    let result: i64 = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| eval(line.as_bytes()))
        .filter_map(Result::ok)
        .sum();
    println!("sum: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closing_parens() {
        let inp = "((2+3) * 4) + 5";
        assert_eq!(10, find_closing_paren(inp.as_bytes()).unwrap());
        assert_eq!(4, find_closing_paren(&inp.as_bytes()[1..]).unwrap());
    }

    #[test]
    fn test_index_past_number() {
        assert_eq!(1, index_past_number("9".as_bytes()));
        assert_eq!(2, index_past_number("99 +".as_bytes()));
        assert_eq!(4, index_past_number("3570)".as_bytes()));
        assert_eq!(4, index_past_number("3570".as_bytes()));
    }
    #[test]
    fn test_eval() {
        assert_eq!(2, eval("(2)".as_bytes()).unwrap());
        assert_eq!(3, eval("1 + 2".as_bytes()).unwrap());
        assert_eq!(20, eval("2 * 3 + 7".as_bytes()).unwrap());
        assert_eq!(
            1445,
            eval("5 + (8 * 3 + 9 + 3 * 4 * 3)".as_bytes()).unwrap()
        );
        assert_eq!(51, eval("1 + (2 * 3) + (4 * (5 + 6))".as_bytes()).unwrap());
        assert_eq!(46, eval("2 * 3 + (4 * 5)".as_bytes()).unwrap());
        assert_eq!(
            1445,
            eval("5 + (8 * 3 + 9 + 3 * 4 * 3)".as_bytes()).unwrap()
        );
        assert_eq!(
            669060,
            eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".as_bytes()).unwrap()
        );
        assert_eq!(
            23340,
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".as_bytes()).unwrap()
        );
    }
}
