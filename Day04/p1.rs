use std::fs;

fn convert_txt_to_matrix(input: &str) -> Vec<Vec<char>> {
    
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn main() {
    let input = fs::read_to_string("day4_input.txt")
        .expect("Should have been able to read the file");

    let matrix: Vec<Vec<char>> = convert_txt_to_matrix(&input);

    let mut r: usize = 0;
    let mut c: usize;
    let mut cnt: i16 = 0;

    for row in &matrix {
        c = 0;
        for _ in row {
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
                    cnt += 1;
                }
            }

            c += 1;
        }
        r += 1;
    }

    println!("Result: {}", cnt);
}