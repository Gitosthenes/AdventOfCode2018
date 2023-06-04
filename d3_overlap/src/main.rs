use regex::Regex;
use std::{
    fs,
    io::{BufRead, BufReader},
};

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() {
    let claims: Vec<Claim> = get_input_as_vec();
    let mut fabric: [[i32; 1000]; 1000] = [[0i32; 1000]; 1000];

    // Mark each sq. inch of fabric with #claims
    for claim in &claims {
        let end_x = claim.x + claim.w;
        let end_y = claim.y + claim.h;

        for i in claim.x..end_x {
            for j in claim.y..end_y {
                fabric[i as usize][j as usize] += 1;
            }
        }
    }

    // Find claim with no overlap
    for claim in &claims {
        let end_x = claim.x + claim.w;
        let end_y = claim.y + claim.h;
        let mut found = true;

        for i in claim.x..end_x {
            for j in claim.y..end_y {
                if fabric[i as usize][j as usize] != 1 {
                    found = false;
                    break;
                }
            }

            if !found {
                break;
            }
        }

        if found {
            println!("{}", claim.id);
            break;
        }
    }
}

fn get_input_as_vec() -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();

    // "#{id} @ {x},{y}: {w}x{h}"
    static PATTERN: &str = r"#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<w>\d+)x(?P<h>\d+)";
    let rgx = Regex::new(PATTERN).expect("Failed to build regex.");

    let file = fs::File::open("input.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let text = line.expect("Failed to read line.");
        let captures = rgx.captures(&text).expect("Failed to retrieve captures.");

        claims.push(Claim {
            id: captures
                .get(1)
                .expect("Failed to get `id` from capture.")
                .as_str()
                .to_owned()
                .parse::<i32>()
                .expect(""),
            x: captures
                .get(2)
                .expect("Failed to get `x` from capture.")
                .as_str()
                .to_owned()
                .parse::<i32>()
                .expect(""),
            y: captures
                .get(3)
                .expect("Failed to get `y` from capture.")
                .as_str()
                .to_owned()
                .parse::<i32>()
                .expect(""),
            w: captures
                .get(4)
                .expect("Failed to get `w` from capture.")
                .as_str()
                .to_owned()
                .parse::<i32>()
                .expect(""),
            h: captures
                .get(5)
                .expect("Failed to get `z` from capture.")
                .as_str()
                .to_owned()
                .parse::<i32>()
                .expect(""),
        });
    }

    claims
}

/* Part 1: Count sq. inches where #claims > 1

   let mut num_overlaps = 0;
   for i in 0..1000 {
       for j in 0..1000 {
           if fabric[i as usize][j as usize] > 1 {
               num_overlaps += 1;
           }
       }
   }

   println!("{}", num_overlaps);
*/
