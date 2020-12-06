use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::string::String;
use std::{env, io};

// The output is wrapped in a Result to allow matching on errors.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_lines(
    lines_it: io::Lines<io::BufReader<File>>,
    part_two: bool,
) -> Result<usize, Box<dyn Error>> {
    let mut total = 0;
    let mut uniq = HashSet::new();
    let mut empty_intersection: bool = false;
    for maybe_line in lines_it {
        let line = match maybe_line {
            Ok(line) => line,
            Err(_) => return Err("unable to read line".into()),
        };
        if line.trim().len() == 0 {
            // end of record, add set count to total, reset
            total += uniq.len();
            uniq.clear();
            empty_intersection = false;
            continue;
        }
        if part_two && empty_intersection {
            continue; // this record has no questions to which everyone answered "yes"
        }

        if uniq.len() == 0 {
            // first line of record
            uniq = line.bytes().collect();
            continue;
        }
        let newset = line.bytes().collect(); // unique letters in this line
        if part_two {
            uniq = uniq.intersection(&newset).map(|p| *p).collect();
            // if the intersection is empty, set flag to signal empty intersection of record lines
            if uniq.len() == 0 {
                empty_intersection = true;
            }
        } else {
            // part one - set union
            uniq = uniq.union(&line.bytes().collect()).map(|p| *p).collect();
        }
    }
    return Ok(total + uniq.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {:?} <inputfile>", args[0]);
    }
    let inputfile = &args[1];
    let it = read_lines(inputfile).unwrap();
    match process_lines(it, false) {
        Err(why) => panic!("failed to process lines for part one: {:?}", why),
        Ok(total) => println!("part one: {:?}", total),
    };

    let it = read_lines(inputfile).unwrap();
    let total = match process_lines(it, true) {
        Err(why) => panic!("failed to process lines for part two: {:?}", why),
        Ok(total) => total,
    };
    println!("part two: {:?}", total)
}
