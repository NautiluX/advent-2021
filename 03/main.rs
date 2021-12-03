use std::io::{self, BufRead};

fn main() {
    let mut linecount = 0;
    let mut sums = Vec::new();
    let mut mask: String = "".to_owned();
    for line in io::stdin().lock().lines() {
        linecount = linecount + 1;
        let current_line = line.unwrap();
        for i in 0..current_line.chars().count() {
            if sums.len() <= i {
                sums.push(current_line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap());
                mask = mask + "1";
                continue;
            }
            sums[i] = sums[i] + current_line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap();
        }
    }
    let mut gamma = "".to_string();

    for s in sums {
        gamma = gamma + &(s as f64/linecount as f64).round().to_string();
    }

    let gammaint = isize::from_str_radix(&gamma, 2).unwrap();
    let maskint = isize::from_str_radix(&mask, 2).unwrap();
    println!("{}", gammaint*(gammaint^maskint))
}