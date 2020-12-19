use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn add_rule<'a>(line: &'a str, rules: &mut HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut s = line.splitn(2, ": ");
    let key = s.next().ok_or("bad input / key")?.to_owned();
    let val = s.next().ok_or("bad input / value")?.to_owned();
    rules.insert(key, val);
    Ok(())
}

fn make_regexp_wrap(rules: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let mut re = String::new();
    re.push_str("^");
    re.push_str(&make_regexp(&"0", rules)?);
    re.push_str("$");
    Ok(re)
}

fn make_regexp(ind: &str, rules: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if ind == "11" || ind == "42" {
        println!("ind={}", ind);
    }
    let rule = rules.get(ind).ok_or("no key")?;
    if rule == "\"a\"" {
        return Ok("a".to_owned());
    }
    if rule == "\"b\"" {
        return Ok("b".to_owned());
    }

    let mut regexp = String::new();
    // if ind == "11" {
    // 	let a = make_regexp("42", rules)?;
    // 	let b = make_regexp("31", rules)?;
    // 	regexp.push_str("(");
    // 	regexp.push_str(&a);
    // 	regexp.push_str(")");    	
    // 	regexp.push_str(&"{2}");  // N
    // 	regexp.push_str("(");
    // 	regexp.push_str(&b);
    // 	regexp.push_str(")");    	
    // 	regexp.push_str(&"{2}");  // N
    // 	println!("11: {}", regexp);
    // 	return Ok(regexp);
    // }
    if !rule.contains("|") {
        for s in rule.split_ascii_whitespace() {
            let part = make_regexp(s, rules)?;
            regexp.push_str(&part);
        }
        // if ind == "42" {
        //     regexp.push_str("+");
        //     println!("42: {}", regexp);
        // }
        return Ok(regexp);
    }
    let mut sep = "";
    regexp.push_str("(");
    for p in rule.split("|") {
        regexp.push_str(sep);
        for s in p.split_ascii_whitespace() {
            let part = make_regexp(s, rules)?;
            regexp.push_str(&part);
        }
        sep = "|";
    }
    regexp.push_str(")");
    // if ind == "42" {
    //     regexp.push_str("+");
    //     println!("42: {}", regexp);
    // }
    return Ok(regexp);
}

enum State {
    Part1,
    Part2,
}

pub fn main() {
    let mut rules: HashMap<String, String> = HashMap::new();
    let mut cnt_valid = 0;
    let mut state = State::Part1;
    let mut rexp: String;
    let mut re = Regex::new("").unwrap();

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        match state {
            State::Part1 => {
                if line.len() == 0 {
                    state = State::Part2;
                    rexp = make_regexp_wrap(&rules).unwrap();
                    println!("regexp: {:?}", rexp);
                    re = Regex::new(&rexp).unwrap();
                    continue;
                }
                add_rule(&line, &mut rules).unwrap();
                continue;
            }
            State::Part2 => {
                if re.is_match(&line) {
                    // println!("{} match", line);
                    cnt_valid += 1;
                } else {
                    // println!("{} no match", line);
                }
            }
        };
    }
    println!("count: {}", cnt_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rule() {
        let mut rules: HashMap<String, String> = HashMap::new();
        let inp = "1: 2 3 | 3 2";
        add_rule(&inp, &mut rules).expect("failed to add rule");
        assert_eq!(1, rules.len());
    }
}
