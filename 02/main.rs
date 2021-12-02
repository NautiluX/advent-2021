use std::io::{self, BufRead};

fn main() {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;
    for line in io::stdin().lock().lines() {
        let str = line.unwrap();
        let v: Vec<&str> = str.split_whitespace().collect();
        let command = v[0];
        let num = v[1].parse::<i32>().unwrap();
        match command {
            "up" => aim = aim - num,
            "down" => aim = aim + num,
            "forward" => {
                horizontal = horizontal + num;
                depth = depth+num*aim;
            },
            &_ => panic!("unknown command"),
        }
    }
    println!("{}", depth*horizontal)
}