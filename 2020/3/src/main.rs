use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let mut tob_map = Vec::new();

    let mut width = 0;
    let mut max = 0;

    if let Ok(input_lines) = read_lines("./input") {
        for line in input_lines {
            let mut width_counter = 0;

            for ch in line.unwrap().chars(){
                tob_map.push(ch);
                width_counter += 1;
                max += 1;
            }
            
            if width < width_counter {
                width = width_counter;
            }
        }
    }

    let trees = calculate_trees(3, 1, &tob_map, &width, &max);
    println!("right: {}; down: {}; trees = {}", 3, 1, trees);

    let tree_seq = vec!((1,1),(3,1),(5,1),(7,1),(1,2));

    let mut tree_tot: i64 = 1;
    for seq in tree_seq {
        let tree_count = calculate_trees(seq.0, seq.1, &tob_map, &width, &max);
        println!("right: {}; down: {}; trees = {}", seq.0, seq.1, tree_count);
        tree_tot = tree_tot * tree_count;
    }

    println!("tree tot: {}", tree_tot);
}

fn calculate_trees(slope_right: usize, slope_down: usize, map: &Vec<char>, map_width: &usize, end_of_map: &usize) -> i64 {
    let mut current_pos: usize = 0;
    let mut tree_count: i64 = 0;

    while current_pos <= *end_of_map {
        if map[current_pos] == '#'{
            tree_count += 1;
        }

        current_pos = move_right(slope_right, current_pos, map_width);
        current_pos = move_down(slope_down, current_pos, map_width);
    }

    tree_count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn move_down(steps: usize, current_pos: usize, width: &usize) -> usize {
    current_pos + (steps * width)
}

fn move_right(steps: usize, current_pos: usize, width: &usize) -> usize {
    let new_pos = current_pos + steps;

    //did move fall off the edge?
    if (new_pos / width) - (current_pos / width) != 0 {
        return  new_pos - width
    }

    new_pos
}