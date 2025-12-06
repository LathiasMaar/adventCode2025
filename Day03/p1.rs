use std::fs;

fn main() {

    let file = fs::read_to_string("day3_input.txt")
        .expect("Should have been able to read the file");

    let banks: Vec<&str> = file.lines().collect();

    let mut joltage: i16 = 0;

    for batteries in banks {
        let volts: Vec<char> = batteries.chars().collect();
        
        let mut voltage = 0;

        for b1 in 0..volts.len() {
            let m1 = volts[b1].to_digit(10).unwrap() as i16;
            for b2 in b1+1..volts.len() {
                let m2 = volts[b2].to_digit(10).unwrap() as i16;
                if m1*10 + m2 > voltage {    
                    voltage = m1*10 + m2;
                }
            }
        }
        joltage += voltage;
    }

    println!("answ: {}", joltage);
}