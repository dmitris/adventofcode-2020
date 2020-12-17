use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::io::BufRead;

type PointX = (i32,i32,i32,i32);

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
        let mut cands : HashSet<PointX> = HashSet::new();
        for p in &self.recs {
            for x in (p.0)-1..(p.0)+2 {
                for y in (p.1)-1..(p.1)+2 {
                    for z  in (p.2)-1..(p.2)+2 {
                        for w  in (p.3)-1..(p.3)+2 {
                            cands.insert((x,y,z,w));
                        }
                    }
                }
            }
        }
    
        let mut addlist : Vec<PointX> = Vec::new();
        let mut removelist : Vec<PointX> = Vec::new();
        for p in cands {
            let active = self.recs.contains(&p);
            let n = self.check_neighbors(&p);
            match (active, n) {
                (true, 2) | (true, 3) => (),
                (true, _) => removelist.push(p),
                (false, 3) => addlist.push(p),
                (false, _) => (),
            };
        }
        for p in removelist.iter() {
            self.recs.remove(p);
        }    
        for p in addlist.into_iter() {
            self.recs.insert(p);
        }
        self.count()
    }
    fn check_neighbors(&self, p : &PointX) -> u32 {
        let mut cnt = 0;
        for x in (p.0)-1..(p.0)+2 {
            for y in (p.1)-1..(p.1)+2 {
                for z  in (p.2)-1..(p.2)+2 {
                    for w  in (p.3)-1..(p.3)+2 {
                        // skip own coordinates
                        if x == p.0 && y == p.1 && z == p.2 && w == p.3 {
                            continue
                        }
                        if self.recs.contains(&(x,y,z,w)) {
                            cnt += 1;
                        }
                        if cnt > 3 {
                            return cnt;
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
            b'#' => res.push((rowidx as i32,i as i32,0,0)),
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
