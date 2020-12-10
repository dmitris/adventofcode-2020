use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

const SUM: u32 = 2020;

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

// find_two reads the given lines iterator and converts each line to integer.
// Uses a HashSet to store numbers - if for a given number n, we already find
// SUM - n in the set, we are done, return the product; otherwise add n to the HashSet.
#[allow(dead_code)]
fn find_two(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut numbers = HashSet::new();

    for line in lines {
        match line {
            Err(err) => panic!("Could not read line: {:?}", err),
            Ok(string) => match string.trim().parse::<u32>() {
                Err(err2) => panic!("Not a number! {:?}", err2),
                Ok(num) => {
                    if numbers.contains(&(SUM - num)) {
                        return num * (SUM - num);
                    }
                    // verbose output for Debug / Info
                    // println!("{:?} not in HashSet, inserting {:?}", SUM-num, num);
                    numbers.insert(num);
                }
            },
        }
    }
    panic!(
        "no numbers summing to {:?} found, please check the input!",
        SUM
    );
}

// find_three reads the given lines iterator and converts each line to integer.
// Use a HashSet to store numbers - if for a given number n, we already find
// SUM - n in the set, we are done, return the product; otherwise add n to the HashSet.
fn find_three(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut vec: Vec<u32> = Vec::new();

    for line in lines {
        match line {
            Err(err) => panic!("Could not read line: {:?}", err),
            Ok(string) => match string.trim().parse::<u32>() {
                Err(err2) => panic!("Not a number! {:?}", err2),
                Ok(num) => vec.push(num),
            },
        };
    }
    for i in 1..vec.len() {
        for j in i..vec.len() {
            for k in i..vec.len() {
                // verbose output for Debug / Info
                // println!("i={:?}, j={:?}, {:?} + {:?} + {:?} = {:?}",
                //          i, j, vec[i], vec[j], vec[k], vec[i]+vec[j]+vec[k]);
                if vec[i] + vec[j] + vec[k] == SUM {
                    return vec[i] * vec[j] * vec[k];
                };
            }
        }
    }
    panic!(
        "no three numbers summing to {:?} found, please check the input!",
        SUM
    );
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {:?} <inputfile>", args[0]);
    }
    let inputfile = &args[1];
    match read_lines(inputfile) {
        Err(err) => panic!("failed to read lines from {:?}: {:?}", inputfile, err),
        // part 1
        // Ok(lines) => println!("{:?}", find_two(lines)),
        // part 2
        Ok(lines) => println!("{:?}", find_three(lines)),
    };
}
