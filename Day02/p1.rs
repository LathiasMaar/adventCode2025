use std::fs;


fn check_vld_id(id:&str) -> bool {
    let (first_half, second_half) = id.split_at(id.len() / 2);
    if first_half == second_half {
        return true;
    }
    return false;
}


fn main() {

    let file = fs::read_to_string("day2_input.txt")
        .expect("Should have been able to read the file");

    let ranges: Vec<&str> = file.split(",").collect();

    let mut ans = 0;

    for r in ranges {
        let limits: Vec<&str> = r.split("-").collect();
        let start: u64 = limits[0].parse().unwrap();
        let end: u64 = limits[1].parse().unwrap();

        for id in start..=end {
            let id_str = id.to_string();
            if check_vld_id(&id_str) {
                ans += id;
            }
        }
    }   
    println!("The solution is: {}", ans);
}