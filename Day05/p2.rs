use std::fs;


fn main() {
    let input = fs::read_to_string("day5_input.txt")
        .expect("Should have been able to read the file");
    
    let (interval_list, id_list) = input.split_once("\n\n").unwrap();  

    let mut intervals: Vec<_> = interval_list
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            [start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap()]
        })
        .collect();

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut count = 0;
    let mut pointer = 0;

    for interval in &mut intervals{
        
        if pointer > interval[1]{
            continue;
        } else if pointer >= interval[0] && pointer <= interval[1]{
            count += interval[1] - pointer;
            pointer = interval[1];
        } else if pointer < interval[0]{
            count += interval[1] - interval[0] + 1;
            pointer = interval[1];
        }
    }

    println!("{}", count)
    
}