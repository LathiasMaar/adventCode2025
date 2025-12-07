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
        
    let ids: Vec<usize> = id_list
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut count = 0;

    for id in ids{

        for interval in &mut intervals{
            if id >= interval[0] && id <= interval[1]{
                count += 1;
                break;
            }
            if id > interval[0]{
                continue;
            }
        }
    }

    println!("{}", count)
    
}