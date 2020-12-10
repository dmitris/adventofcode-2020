use std::error::Error;
use std::io::BufRead;

use daggy::Dag;
use regex::Regex;

#[derive(Debug)]
struct BagRec {
    color : String,
    num : u32, // quantity of bags of this color allowed
}

// const MIN_TOKENS : usize = 7; // minimal number of space-separated tokens in a line
fn parse_line(line: &str) -> Result<(std::string::String, Option<Vec<BagRec>>), Box<dyn Error>> {
    if line.ends_with(" bags contain no other bags.") {
        let parts : Vec<&str> = line.splitn(3, " ").collect();
        let c1 = parts[0..2].join(" ");
        println!("LEAF {:?} - line: {:?}", c1, line);
        return Ok((c1, None));
    }

    let rstr = r"^(?P<c1>[[:alpha:]]+\s[[:alpha:]]) contain ";
    let _re = Regex::new(rstr).unwrap();
    // Ok( ("endofrec".into(), None))

    // line.split(' ').collect();
    // if tkns.len() < MIN_TOKENS {
    //     return Err("line has two few tokesn".into());
    // }
    // for (i, p) in tkns.iter().enumerate() {
    //     println!("{:?} => {:?}", i, p);
    // }
    // let c1 = tkns[0..2].join(" ");
    // let num = match  tkns[4].parse::<u32>() {
    //     Err(_) => return Err("bad input - no index in quantity of bags".into()),
    //     Ok(n) => n,
    // };
    // let c2 = match tkns.len() {
    //     7 => None,
    //     _ => Some(tkns[5..7].join(" ")),
    // };
    // println!("{:?}: {:?}, {:?}, {:?}", line, c1, c2, num);
    // Ok((c1, c2, num))
}

fn main() {
    let res = parse_line("light salmon bags contain no other bags.").unwrap();
    println!("{:?}", res);
    // let _dag: Dag<&str, &str> = Dag::new();

    // let _records: Vec<(String, Option<Vec<BagRec>>)> = std::io::stdin()
    //     .lock()
    //     .lines()
    //     .filter_map(Result::ok)
    //     .map(|l| parse_line(&l))
    //     .filter_map(Result::ok)
    //     .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let res = parse_line("light salmon bags contain no other bags.").unwrap();
        assert_eq!(res.0, "light salmon");
        assert!(res.1.is_none());
        let res = parse_line("light salmon bags contain 5 dotted olive bags, 4 wavy lavender bags").unwrap();
        assert_eq!(res.0, "light salmon");
    }
}