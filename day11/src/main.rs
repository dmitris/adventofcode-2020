use std::io::BufRead;

enum SeatType (
    Occupied
    Empty
    Floor
);

struct Rec
fn main() {
    let recs = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line2rec(line)).collect();
    println!("Hello, world!");
}
