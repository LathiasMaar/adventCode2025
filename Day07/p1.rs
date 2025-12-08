use std::fs;

fn convert_txt_to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn main() {
    let input = fs::read_to_string("day7_input.txt")
        .expect("Should have been able to read the file");
    
    let mut map: Vec<Vec<char>> = convert_txt_to_matrix(&input);
    let mut cnt: i16 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if i == map.len() - 1 {
                break;
            }
            if map[i][j] == 'S' {
                map[i+1][j] = '|';
            } 
            if map[i][j] == '|' {
                if map[i+1][j] == '.' {
                    map[i+1][j] = '|';
                } else if map[i+1][j] == '^' {
                    if map[i+1][j-1] == '.' {
                        map[i+1][j-1] = '|';
                    }
                    if map[i+1][j+1] == '.' {
                        map[i+1][j+1] = '|';
                    }
                    cnt += 1;
                }
            }
        }
    }
    
    println!("Answer: {}", cnt);
}