use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::string::String;
use std::{env, io};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// Function copied from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const LINELEN: usize = 10;
fn pass2id(line: &str) -> Result<u16, Box<dyn Error>> {
    if line.len() != LINELEN {
        return Err("invalid input - str length != 11".into());
    }
    let mut num: u16 = 0;
    for i in 0..LINELEN {
        // println!("line={:?}, i={:?}, num={:?}", line, i, num);
        match line.as_bytes()[i] {
            b'B' | b'R' => num |= 1 << (LINELEN - 1 - i) as u16,
            b'F' | b'L' => (),
            _ => return Err("invalid input - unknown character".into()),
        };
    }
    return Ok(num);
}

fn find_max_and_empty(it: io::Lines<io::BufReader<File>>) -> Result<(u16, u16), Box<dyn Error>> {
    let mut min: u16 = u16::MAX;
    let mut max: u16 = 0;
    // (2^(n+1)) - 1 where n = LINELEN
    let num_seats = (1 << (LINELEN + 1)) - 1;
    let mut seats = vec![false; num_seats];
    for next in it {
        let line = match next {
            Err(why) => panic!(why),
            Ok(line) => line,
        };
        let id = match pass2id(&line) {
            Ok(id) => id,
            Err(why) => {
                println!("{:?}", why);
                panic!("invalid input");
            }
        };
        if id > max {
            max = id;
        }
        if id < min {
            min = id;
        }
        seats[id as usize] = true;
    }
    let empty = find_empty(&seats, min)?;
    return Ok((max, empty));
}

fn find_empty(seats: &Vec<bool>, min: u16) -> Result<u16, Box<dyn Error>> {
    for i in (min as usize)..seats.len() {
        if seats.get(i) == Some(&false) {
            return Ok(i as u16);
        }
    }
    return Err("no empty seat found".into());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {:?} <inputfile>", args[0]);
    }
    let inputfile = &args[1];
    let it = match read_lines(inputfile) {
        Ok(lines_it) => lines_it,
        Err(err) => panic!("failed to read lines from {:?}: {:?}", inputfile, err),
    };
    let (max, empty) = match find_max_and_empty(it) {
        Err(err) => panic!("bad input: {:?}", err),
        Ok((a, b)) => (a, b),
    };
    println!("part 01: {:?}", max);
    println!("part 02: {:?}", empty);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass2id() {
        assert_eq!(pass2id("FFFFFFFLLL").unwrap(), 0);
        assert_eq!(pass2id("FFFFFFFLLR").unwrap(), 1);
        assert_eq!(pass2id("FFFFFFFLRL").unwrap(), 2);
        assert_eq!(pass2id("BFFFBBFRRR").unwrap(), 567);
        assert_eq!(pass2id("FFFBBBFRRR").unwrap(), 119);
        assert_eq!(pass2id("BBFFBBFRLL").unwrap(), 820);
    }

    #[test]
    fn test_find_empty() {
        let seats = vec![false, false, true, true, false, true]; // empty seat: 4
        assert_eq!(find_empty(&seats, 2).unwrap(), 4);
    }
}
