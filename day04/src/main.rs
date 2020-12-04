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

fn count_valid(it: io::Lines<io::BufReader<File>>, part2: bool) -> Result<u32, Box<dyn Error>> {
    let mut count = 0;
    let mut check: u8 = 0;
    let mut valid = true;
    for next in it {
        let line = match next {
            Err(why) => return Err(Box::new(why)),
            Ok(line) => line,
        };
        if !line.trim().is_empty() {
            if part2 {
                match process_validate_line(&line, check) {
                    Err(_) => valid = false,
                    Ok(c) => check = c,
                }
            } else {
                check = process_line(&line, check)?;
            }
            continue;
        }
        // new line - end of record
        if valid && check >= 0b1111_1110 {
            // all bit flags set except possibly the 0x1 = "cid"
            count += 1;
        }
        valid = true;
        check = 0;
    }
    // final check after the last line
    if valid && check >= 0b1111_1110 {
        Ok(count + 1)
    } else {
        Ok(count)
    }
}

// part one processing
fn process_line(line: &str, check: u8) -> Result<u8, Box<dyn Error>> {
    let mut ret = check;
    for kv in line.split_whitespace() {
        if kv.len() < 5 {
            return Err("invalid input - short key-value pair".into());
        };
        match &kv[0..3] {
            "byr" => ret |= 0b1000_0000,
            "iyr" => ret |= 0b0100_0000,
            "eyr" => ret |= 0b0010_0000,
            "hgt" => ret |= 0b0001_0000,
            "hcl" => ret |= 0b0000_1000,
            "ecl" => ret |= 0b0000_0100,
            "pid" => ret |= 0b0000_0010,
            "cid" => ret |= 0b0000_0001,
            key => return Err(["invalid input - unknown key: ", key].join("").into()),
        };
    }
    Ok(ret)
}

// part two processing
fn process_validate_line(line: &str, check: u8) -> Result<u8, Box<dyn Error>> {
    let mut ret = check;
    for kv in line.split_whitespace() {
        let mut kv_iter = kv.split(":");
        let key = match kv_iter.next() {
            Some(s) => s,
            None => return Err("bad input".into()),
        };
        let val = match kv_iter.next() {
            Some(s) => s,
            None => return Err("bad input".into()),
        };
        ret = check_key_values(key, val, ret)?;
    }
    Ok(ret)
}

fn check_key_values(key: &str, val: &str, check: u8) -> Result<u8, Box<dyn Error>> {
    match key {
        "byr" => {
            let year = val.parse::<u16>()?;
            if year >= 1920 && year <= 2002 {
                return Ok(check | 0b1000_0000);
            };
        }
        "iyr" => {
            let year = val.parse::<u16>()?;
            if year >= 2010 && year <= 2020 {
                return Ok(check | 0b0100_0000);
            };
        }
        "eyr" => {
            let year = val.parse::<u16>()?;
            if year >= 2020 && year <= 2030 {
                return Ok(check | 0b0010_0000);
            };
        }
        "hgt" => {
            let height: u16 = val[0..val.len() - 2].parse::<u16>()?;
            let units = &val[val.len() - 2..val.len()];
            match units {
                "cm" => {
                    if height >= 150 && height <= 193 {
                        return Ok(check | 0b0001_0000);
                    }
                }
                "in" => {
                    if height >= 59 && height <= 76 {
                        return Ok(check | 0b0001_0000);
                    }
                }
                _ => return Err("invalid input - bad units in the hgt field".into()),
            };
        }
        "hcl" => {
            let val_bytes = val.as_bytes();
            if val_bytes.len() != 7 || val_bytes[0] != '#' as u8 {
                return Err("invalid input - no # at the start of the hcl value".into());
            }
            for i in 1..7 {
                let i = i as usize;
                if (val_bytes[i] >= '0' as u8 && val_bytes[i] <= '9' as u8)
                    || (val_bytes[i] >= 'a' as u8 && val_bytes[i] <= 'f' as u8)
                {
                    // OK
                } else {
                    return Err("invalid input - invalid characters in the hcl value".into());
                }
            }
            return Ok(check | 0b0000_1000);
        }
        "ecl" => {
            match val {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                    return Ok(check | 0b0000_0100)
                }
                _ => return Err("invalid input - bad ecl value".into()),
            };
        }
        "pid" => {
            if val.len() != 9 {
                return Err("invalid input - wrong length of value of pid field".into());
            }
            if val.chars().all(char::is_numeric) {
                return Ok(check | 0b0000_0010);
            }
        }
        "cid" => return Ok(check | 0b0000_0001),
        _ => return Err("bad input - invalid key name".into()),
    };
    // println!("DMDEBUG: failure key={:?}, val={:?}, check={:b}", key, val, check);
    Err("invalid input - failed key/value validation".into())
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
    // part one
    match count_valid(it, false) {
        Err(why) => panic!("failed to count valid passports: {:?}", why),
        Ok(num) => println!("part one: {:?}", num),
    }
    // part two
    let it = read_lines(inputfile).unwrap();
    match count_valid(it, true) {
        Err(why) => panic!("failed to count valid passports: {:?}", why),
        Ok(num) => println!("part two: {:?}", num),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn valid_key_value() {
        assert!(process_validate_line("byr:2002", 0).is_ok());
        assert!(process_validate_line("hgt:60in", 0).is_ok());
        assert!(process_validate_line("hgt:190cm", 0).is_ok());
        assert!(process_validate_line("hcl:#123abc", 0).is_ok());
        assert!(process_validate_line("ecl:brn", 0).is_ok());
        assert!(process_validate_line("pid:000000001", 0).is_ok());
    }

    #[test]
    fn invalid_key_value() {
        assert!(process_validate_line("byr:2003", 0).is_err());
        assert!(process_validate_line("hgt:190", 0).is_err());
        assert!(process_validate_line("hcl:#123abz", 0).is_err());
        assert!(process_validate_line("hcl:123abc", 0).is_err());
        assert!(process_validate_line("ecl:wat", 0).is_err());
        assert!(process_validate_line("pid:0123456789", 0).is_err());
    }

    #[test]
    fn valid_passports() {
        let inp01 = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert_eq!(0b1111_1110, process_validate_line(inp01, 0).unwrap());
        let inp02 =
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        assert_eq!(0b1111_1111, process_validate_line(inp02, 0).unwrap());
        let inp03 = "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022";
        assert_eq!(0b1111_1111, process_validate_line(inp03, 0).unwrap());
        let inp04 = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(0b1111_1110, process_validate_line(inp04, 0).unwrap());
    }

    #[test]
    fn invalid_passports() {
        let inputs = vec![
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        ];
        for inp in inputs.iter() {
            assert!(process_validate_line(inp, 0).is_err());
        }
    }
}
