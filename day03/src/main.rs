use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

// Returns a Vector of the non-empty lines from the file, wrapped in a Result.
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut vec: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        match line {
            Err(err) => println!("failed to read line: {:?}", err),
            Ok(line) if line.len() > 0 => vec.push(line),
            _ => (),
        }
    }
    Ok(vec)
}

// ensure all the lines are the same length
fn validate_input(lines: &Vec<String>) -> Result<(), Box<dyn Error>> {
    if lines.len() == 0 {
        return Err("Error - empty input file".into());
    }
    let linelen = lines[0].len();
    for i in 0..lines.len() {
        if lines[i].len() != linelen {
            return Err("bad input".into());
        }
    }
    Ok(())
}

fn count_trees(lines: &Vec<String>, (right, down): (usize, usize)) -> Result<u32, Box<dyn Error>> {
    let n = lines[0].len() as usize; // length of the first and therefore every line
    let mut ind = right;
    let mut trees: u32 = 0;
    // start with line number <down> and then skip down by the same parameter (<down>)
    let it = down..lines.len();
    for i in it.step_by(down) {
        match lines[i].as_bytes()[ind] as char {
            '#' => trees += 1,
            '.' => (),
            _ => return Err("bad input - illegal character found".into()),
        }
        ind = (ind + right) % n;
    }
    Ok(trees)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {:?} <inputfile>", args[0]);
    }
    let inputfile = &args[1];
    let lines = read_lines(inputfile);
    let lines = match lines {
        Ok(lines) => lines,
        Err(err) => panic!("Failed to read lines from file {:?} {:?}", inputfile, err),
    };
    match validate_input(&lines) {
        Err(why) => panic!("{:?}", why),
        _ => (),
    };
    // part one
    match count_trees(&lines, (3, 1)) {
        Err(why) => panic!("{:?}", why),
        Ok(n) => println!("part one: {:?}", n),
    }
    // part two
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut answer = 1;
    for &slope in slopes.iter() {
        answer = answer * count_trees(&lines, slope).unwrap();
    }
    println!("part two: {:?}", answer);
}
