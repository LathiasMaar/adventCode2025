use std::fs;

fn main() {

    let file = fs::read_to_string("day1_input.txt")
        .expect("Should have been able to read the file");

    let instructions: Vec<&str> = file.lines().collect();

    let mut pos: i16 = 50;
    let mut cnt: i16 = 0;

    for instruction in instructions {
        let dir = &instruction[0..1];
        let dist: i16 = instruction[1..].parse().unwrap();


        if dir == "R" {
            pos = (pos + dist) % 100;
        } else if dir == "L" {
            pos = (pos - dist+100) % 100;
        } 

        if pos == 0 {
            cnt += 1;
        }
    }

    println!("psswd PART1: {}", cnt);
}