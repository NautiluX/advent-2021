use std::io::{self, BufRead};

fn main() {
    let mut previous_int1 = 0;
    let mut previous_int2 = 0;
    let mut previous_int3 = 0;
    let mut inc = 0;
    for line in io::stdin().lock().lines() {
        let new_int = line.unwrap().parse::<i32>().unwrap();
        let current_window = new_int+previous_int1+previous_int2;
        let previous_window = previous_int1+previous_int2+previous_int3;

        if current_window > previous_window && previous_int1 > 0 && previous_int2 > 0 && previous_int3 > 0 {
            inc=inc+1;
        }
        previous_int3 = previous_int2;
        previous_int2 = previous_int1;
        previous_int1 = new_int;
    }
    println!("{}", inc)
}