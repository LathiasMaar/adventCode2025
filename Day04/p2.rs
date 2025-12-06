use std::fs;

fn convert_txt_to_matrix(input: &str) -> Vec<Vec<char>> {
    
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn remove_rolls(matrix: &mut Vec<Vec<char>>, done: &mut bool, cnt: &mut i16) {

    let mut new_map = matrix.clone();

    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {

            let mut rolls = 0;

            if matrix[r][c] == '@' {

                // top - left side
                if r != 0 && c != 0 {
                    if matrix[r-1][c-1] == '@' {
                        rolls += 1;
                    }
                }

                // top 
                if r != 0 {
                    if matrix[r-1][c] == '@' {
                        rolls += 1;
                    }
                }

                // top - right side
                if r != 0 && c != matrix[0].len() - 1 {
                    if matrix[r-1][c+1] == '@' {
                        rolls += 1;
                    }
                }

                // left side
                if c != 0 {
                    if matrix[r][c-1] == '@' {
                        rolls += 1;
                    }
                }

                // right side
                if c != matrix[0].len() - 1 {
                    if matrix[r][c+1] == '@' {
                        rolls += 1;
                    }
                }

                // bottom - left side
                if r != matrix.len() - 1 && c != 0 {
                    if matrix[r+1][c-1] == '@' {
                        rolls += 1;
                    }
                }

                // bottom
                if r != matrix.len() - 1 {
                    if matrix[r+1][c] == '@' {
                        rolls += 1;
                    }
                }

                // bottom - right side
                if r != matrix.len() - 1 && c != matrix[0].len() - 1 {
                    if matrix[r+1][c+1] == '@' {
                        rolls += 1;
                    }
                }   

                if rolls < 4 {
                    new_map[r][c] = '.';
                    *done = false;
                    *cnt += 1;
                }
            }
        }
    }
    *matrix = new_map;
}

fn main() {
    let input = fs::read_to_string("day4_input.txt")
        .expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<char>> = convert_txt_to_matrix(&input);

    let mut done = false;
    let mut cnt: i16 = 0;

    while done != true {
        done = true;
        remove_rolls(&mut matrix, &mut done, &mut cnt);
    }

    println!("Result: {}", cnt);
}