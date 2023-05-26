use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    // Read input .txt
    let file = fs::File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    //Convert file contents into Vec
    let mut input_as_vec: Vec<i32> = Vec::new();
    
    for line in reader.lines() {
        let parsed_chg = line
        .expect("Failed to read line")
        .parse::<i32>()
        .expect("Failed to parse i32");
    
        input_as_vec.push(parsed_chg);
    }

    // Loop until repeated freq found
    let mut found = false;
    let mut curr_sum = 0;
    let mut prev_sums: HashSet<i32> = HashSet::new();
    
    prev_sums.insert(curr_sum);
    while !found {
        
        for num in &input_as_vec {
            curr_sum += num;

            if !prev_sums.insert(curr_sum) {
                println!("{}", curr_sum);
                found = true;
                break;
            }
        }
    }
}

// TEST INPUTS
//let input_as_vec: Vec<i32> = vec![1, -1];
//let input_as_vec: Vec<i32> = vec![3, 3, 4, -2, -4];
//let input_as_vec: Vec<i32> = vec![7, 7, -2, -7, -4];