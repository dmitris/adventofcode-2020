use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut vec: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        match line {
            Err(err) => println!("failed to read line: {:?}", err),
            Ok(line) => vec.push(line),
        }
    }
    Ok(vec)
}

fn parse_check(lines: &Vec<String>) -> u32 {
    let mut cnt = 0;
    for line in lines.iter() {
        let v: Vec<&str> = line.split(' ').collect();
        let indices: Vec<&str> = v[0].split('-').collect();
        if indices.len() != 2 {
            panic!(
                "bad input - expect <n>-<n> pair of indices, line: {:?}",
                line
            );
        }
        let min = indices[0].parse::<u32>().unwrap();
        let max = indices[1].parse::<u32>().unwrap();
        let freq = v[2].matches(&v[1][0..1]).count() as u32;
        // println!("[{:?}], min={:?}, max={:?}, freq={:?}", line, min, max, freq);
        if freq >= min && freq <= max {
            // println!("FOUND: {:?}", line);
            cnt += 1;
        }
    }

    return cnt;
}

fn part_two(lines: &Vec<String>) -> u32 {
    let mut cnt = 0;
    for line in lines.iter() {
        let v: Vec<&str> = line.split(' ').collect();
        let indices: Vec<&str> = v[0].split('-').collect();
        if indices.len() != 2 {
            panic!(
                "bad input - expect <n>-<n> pair of indices, line: {:?}",
                line
            );
        }
        let ind_first = indices[0].parse::<usize>().unwrap() - 1;
        let ind_second = indices[1].parse::<usize>().unwrap() - 1;
        let subj = v[1].as_bytes()[0];
        if (v[2].as_bytes()[ind_first] == subj && v[2].as_bytes()[ind_second] != subj)
            || (v[2].as_bytes()[ind_first] != subj && v[2].as_bytes()[ind_second] == subj)
        {
            // println!("FOUND: {:?}", line);
            cnt += 1;
        }
    }
    return cnt;
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
    let _num = parse_check(&lines);
    // println!("part one: {:?}", num);
    let num = part_two(&lines);
    println!("part two: {:?}", num);
}
