use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::io::BufRead;

#[derive(Eq, Hash, PartialEq)]
struct PointX {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl fmt::Display for PointX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{},{})", self.x, self.y, self.z,self.w)
    }
}
struct GridX {
    recs: HashSet<PointX>
}

impl GridX {
    fn new(recs_ : HashSet<PointX>) -> GridX {
        GridX{recs: recs_}
    }
    fn count(&self) -> u32 {
        self.recs.len().try_into().unwrap()
    }
    fn cycle(&mut self) -> u32 {
        let mut newrecs : HashSet<PointX> = HashSet::new();
        for p in &self.recs {
            for x in p.x-1..p.x+2 {
                for y in p.y-1..p.y+2 {
                    for z  in p.z-1..p.z+2 {
                        for w  in p.w-1..p.w+2 {
                            let p = PointX{x,y,z,w};
                            let active = self.recs.contains(&p);
                            let n = self.check_neighbors(&p);
                            match (active, n) {
                                (true, 2) | (true, 3) => {newrecs.insert(p);},
                                (true, _) => (), 
                                (false, 3) => {newrecs.insert(p);},
                                (false, _) => (),
                            };
                        }
                    }
                }
            }
        }

        self.recs = newrecs;
        self.count()
    }
    fn check_neighbors(&self, p : &PointX) -> u32 {
        let mut cnt = 0;
        for x in p.x-1..p.x+2 {
            for y in p.y-1..p.y+2 {
                for z  in p.z-1..p.z+2 {
                    for w  in p.w-1..p.w+2 {
                        if cnt > 3 {
                            return cnt
                        }
                        // skip own coordinates
                        if x == p.x && y == p.y && z == p.z && w == p.w{
                            continue
                        }
                        if self.recs.contains(&PointX{x,y,z,w}) {
                            cnt += 1;
                        }
                    }
                }
            }
        }
        cnt
    }
}

fn parse_line(line: &str, rowidx: u32) -> Result<Vec<PointX>, Box<dyn Error>> {
    if line.len() == 0 {
        return Err("empty input line".into());
    }
    let mut res: Vec<PointX> = Vec::new();
    for (i, b) in line.as_bytes().iter().enumerate() {
        match b {
            b'#' => res.push(PointX {
                x: rowidx as i32,
                y: i as i32,
                z: 0,
                w: 0,
            }),
            b'.' => (),
            _ => return Err("invalid character".into()),
        };
    }
    Ok(res)
}
fn main() {
    let mut cnt: u32 = 0;
    let recs: HashSet<PointX> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let v = parse_line(&l, cnt);
            cnt += 1;
            v
        })
        .filter_map(Result::ok)
        .flatten()
        .collect();
    let mut grid = GridX::new(recs);
    for i in 0..6 { // FIXME - change to 6
        println!("{} - {}", i+1, grid.cycle());
    }
}
