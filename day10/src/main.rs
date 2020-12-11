use std::collections::HashMap;
use std::io::BufRead;

#[allow(dead_code)]
fn part_one(vec: &mut Vec<u32>) -> u32 {
    vec.sort();
    let mut step_one: u32 = 0;
    let mut step_three: u32 = 0;
    for i in 0..vec.len() {
        if i == 0 {
            if vec[0] == 1 {
                step_one += 1;
            } else if vec[0] == 3 {
                step_three += 1;
            }
            continue;
        }
        if &vec[i] - &vec[i - 1] == 1 {
            step_one += 1;
        } else if &vec[i] - &vec[i - 1] == 3 {
            step_three += 1;
        }
    }
    step_one * (step_three + 1)
}

// part two, iterative version
#[allow(dead_code)]
fn part_two(vec: &mut Vec<u32>) -> u64 {
    // add 0 as "the charging outlet"
    vec.push(0);
    vec.sort();
    let mut paths: HashMap<usize, u64> = HashMap::new();
    let hi_ind = vec.len() - 1;
    paths.insert(vec[hi_ind] as usize, 1); // one path from the last element to the "device's built-in adapter"
    for i in (0..vec.len()).rev() {
        let mut acc = 0;
        for j in 1..4 {
            let key = vec[i] + j;
            if !paths.contains_key(&(key as usize)) {
                continue;
            }
            acc += paths.get(&(key as usize)).unwrap();
        }
        if i == hi_ind {
            acc = 1;
        }
        let el = vec[i as usize];
        paths.insert(el as usize, acc);
    }
    return paths[&0];
}

// part two, recursive version with memoization
pub fn part_two_rec(vec: &mut Vec<u32>) -> u64 {
    // add 0 as "the charging outlet"
    if vec.len() == 0 {
        return 0;
    }
    // check for the case vec is already sorted with 0 inserted - ex. with repeated calls
    if vec[0] != 0 {
        vec.push(0);
    }
    vec.sort();
    let mut paths: HashMap<usize, u64> = HashMap::new();
    let hi_ind = vec.len() - 1;
    paths.insert(vec[hi_ind] as usize, 1); // one path from the last element to "device's built-in adapter"
    return part_two_rec_helper(vec, 0, &mut paths);
}

fn part_two_rec_helper(vec: &mut Vec<u32>, ind: usize, paths: &mut HashMap<usize, u64>) -> u64 {
    let mut acc = 0;
    let el = vec[ind] as usize;
    if paths.contains_key(&el) {
        return paths[&el];
    }
    for j in 1..4 {
        let key = ind + j;
        if key >= vec.len() || vec[key] > vec[ind] + 3 {
            break;
        }
        // the next if-clause is a possible optimisation for the "3-steps":
        // if j == 1 && vec[key] == vec[ind] + 3 {
        //     // next step is by three
        //     let next = part_two_rec_helper(vec, key, paths);
        //     paths.insert(el, next);
        //     return next;
        // }
        acc += part_two_rec_helper(vec, key, paths);
    }
    paths.insert(el, acc);
    acc
}

fn main() {
    let mut vec: Vec<u32> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().parse::<u32>())
        .filter_map(Result::ok)
        .collect();
    let res1 = part_one(&mut vec);
    println!("part one: {:?}", res1);
    let res2 = part_two_rec(&mut vec);
    println!("part two: {:?}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut inp1: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part_one(&mut inp1), 35);
        let mut inp2 = vec![
            0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
            35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part_one(&mut inp2), 220);
    }

    #[test]
    fn test_part_two() {
        let mut inp1: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part_two(&mut inp1), 8);
        let mut inp2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part_two(&mut inp2), 19208);
        let mut input_full = vec![
            70, 102, 148, 9, 99, 63, 40, 52, 91, 39, 55, 28, 54, 22, 95, 61, 118, 35, 14, 21, 129,
            82, 137, 45, 7, 87, 81, 25, 3, 108, 41, 11, 145, 18, 65, 80, 115, 29, 136, 42, 97, 104,
            117, 141, 62, 121, 23, 96, 24, 128, 48, 1, 112, 8, 34, 144, 134, 116, 58, 147, 51, 84,
            17, 126, 64, 68, 135, 10, 77, 105, 127, 73, 111, 90, 16, 103, 109, 98, 146, 123, 130,
            69, 133, 110, 30, 122, 15, 74, 33, 38, 83, 92, 2, 53, 140, 4,
        ];
        assert_eq!(part_two(&mut input_full), 49607173328384);
    }

    #[test]
    fn test_part_two_rec() {
        let mut inp1: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part_two_rec(&mut inp1), 8);
        let mut inp2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part_two_rec(&mut inp2), 19208);
        let mut input_full = vec![
            70, 102, 148, 9, 99, 63, 40, 52, 91, 39, 55, 28, 54, 22, 95, 61, 118, 35, 14, 21, 129,
            82, 137, 45, 7, 87, 81, 25, 3, 108, 41, 11, 145, 18, 65, 80, 115, 29, 136, 42, 97, 104,
            117, 141, 62, 121, 23, 96, 24, 128, 48, 1, 112, 8, 34, 144, 134, 116, 58, 147, 51, 84,
            17, 126, 64, 68, 135, 10, 77, 105, 127, 73, 111, 90, 16, 103, 109, 98, 146, 123, 130,
            69, 133, 110, 30, 122, 15, 74, 33, 38, 83, 92, 2, 53, 140, 4,
        ];
        assert_eq!(part_two_rec(&mut input_full), 49607173328384);
    }
}
