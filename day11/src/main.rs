use std::io::BufRead;

#[allow(dead_code)]
enum SeatType {
    Occupied,
    Empty,
    Floor,
}

struct Rec {}

fn line2rec(_line : &str) -> Rec {
    Rec{}
}

fn main() {
    let _recs : Vec<Rec> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line2rec(&line)).collect();
    println!("Hello, world!");
}
