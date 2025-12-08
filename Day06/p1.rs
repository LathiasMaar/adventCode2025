use std::fs;

fn main() {
    let input = fs::read_to_string("day6_input.txt")
        .expect("Should have been able to read the file");
    
    let worksheet: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|tok| tok.to_string())
                .collect::<Vec<String>>()
        })
        .filter(|row| !row.is_empty())  // elimina líneas vacías si las hay
        .collect();
    
    let mut ans:u64 = 0;

    for col in 0..worksheet[0].len()  {
        let mut op = 0;
        let mut value:u64 = 0;

        if worksheet[worksheet.len()-1][col] == "*"{
            op = 1;
            value = 1;
        } else if worksheet[worksheet.len()-1][col] == "+"{
            op = 2;
            value = 0;
        } 

        for row in 0..worksheet.len() - 1 {
            if op == 1 {
                value *= worksheet[row][col].parse::<u64>().unwrap();
            }
            else if op == 2 {
                value += worksheet[row][col].parse::<u64>().unwrap();
            }
        }
        ans += value;
    }
    println!("{}", ans);
}