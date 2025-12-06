use std::fs;

fn get_max_voltage( batteries: &str, n: usize) -> u64{
    
    let volts: Vec<u64> = batteries.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();


    if volts.len() < n {
        return 0;
    }

    let mut max_volt = 0;

    fn recursive_search( volts: &Vec<u64>,      // data vector
                         n: usize,              // size of combination  
                         start: usize,          // starting search               
                         cnt: usize,      // item count in current combination
                         max_volt: &mut u64) {
        
        if cnt == n {
            return;
        }                    

        let mut val = 0;
        let mut idx = 0;

        let remaining = n - cnt - 1;

        for b in start..volts.len() - remaining {
            if volts[b] > val {
                val = volts[b];
                idx = b;
                if val == 9 { break; }
            }
        }

        *max_volt = *max_volt * 10u64 + val as u64;
        recursive_search(volts, n, idx + 1, cnt + 1, max_volt);
    }

    recursive_search(&volts, n, 0, 0, &mut max_volt);
    max_volt
}

fn main() {

    let file = fs::read_to_string("day3_input.txt")
        .expect("Should have been able to read the file");

    let banks: Vec<&str> = file.lines().collect();

    let num = 12;
    let mut joltage: u64 = 0;

    for batteries in banks {
        
        let voltage = get_max_voltage(batteries, num);

        joltage += voltage;
    }

    println!("answ: {}", joltage);
}