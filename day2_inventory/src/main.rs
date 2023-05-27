use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let input = get_input_as_vec();
    let mut correct_id: Vec<char> = Vec::new();

    // Find matching IDs
    for (i, this_id) in input.iter().enumerate() {
        for (j, other_id) in input.iter().enumerate() {
            if i == j {
                continue;
            }

            let matching_chars: Vec<char> = this_id
                .chars()
                .zip(other_id.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect();

            if this_id.len() == matching_chars.len() + 1 {
                correct_id = matching_chars;
                break;
            }
        }

        if correct_id.len() > 0 {
            break;
        }
    }

    // Print asnwer to console
    for chr in correct_id {
        print!("{}", chr);
    }
    println!();
}

fn get_input_as_vec() -> Vec<String> {
    // Read input.txt
    let file = fs::File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    //Convert file contents into Vec
    let mut input_as_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let id = line.expect("Failed to read line");
        input_as_vec.push(id);
    }

    input_as_vec
}

/*  Part 1:

   let input = get_input_as_vec();
   let mut two_count = 0;
   let mut three_count = 0;

   for id in input {
       let mut char_counts: HashMap<char, i32> = HashMap::new();

       for chr in id.chars() {
           let curr_count = char_counts.entry(chr).or_insert(0);
           *curr_count += 1;
       }

       let mut found2 = false;
       let mut found3 = false;
       for count in char_counts.values() {
           if found2 && found3 { break; }

           match count {
               2 => found2 = true,
               3 => found3 = true,
               _ => continue
           }
       }

       if found2 { two_count += 1; }
       if found3 { three_count += 1; }
   }

   println!("{}", two_count * three_count);
*/
