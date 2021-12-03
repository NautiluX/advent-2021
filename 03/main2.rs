use std::io::{self, BufRead};

fn main() {
    let mut input: Vec<Vec<i32>> = Vec::new();
    let mut bitcount = 0;
    for line in io::stdin().lock().lines() {
        let current_line = line.unwrap();

        let mut current_line_vec = Vec::new();
        bitcount = current_line.chars().count();
        for i in 0..current_line.chars().count() {
            current_line_vec.push(current_line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap())
        }
        input.push(current_line_vec);
    }
    let (mut ox, mut co2) = filter_by_bit(&input, 0);
    let mut oxstr = "".to_string()+&ox[0][0].to_string();
    let mut co2str = "".to_string()+&co2[0][0].to_string();
    for i in 1..bitcount {
        if ox.len() > 1 {
            let (oxnew, _co2) = filter_by_bit(&ox, i);
            ox = oxnew;
        }
        if co2.len() > 1 {
            let (_ox, co2new) = filter_by_bit(&co2, i);
            co2 = co2new;
        }
        oxstr = oxstr + &ox[0][i].to_string();
        co2str = co2str + &co2[0][i].to_string();
    println!("ox: {}\nco: {}\n", oxstr, co2str)
    }
    let oxnum = isize::from_str_radix(&oxstr, 2).unwrap();
    let co2num = isize::from_str_radix(&co2str, 2).unwrap();
    println!("{}", oxnum*co2num)
}

fn filter_by_bit(numbers: &Vec<Vec<i32>>, bit: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>){
    let mut ox: Vec<Vec<i32>> = Vec::new();
    let mut co2: Vec<Vec<i32>> = Vec::new();
    let mut bitsum = 0;
    for number in numbers {
        bitsum = bitsum + number[bit];
    }
    let bitsearch = (bitsum as f64 / numbers.len() as f64).round() as i32;

    for number in numbers {
        if number[bit] == bitsearch {
            ox.push(number.clone());
            continue;
        }
        co2.push(number.clone());
    }

    return (ox,co2);

}