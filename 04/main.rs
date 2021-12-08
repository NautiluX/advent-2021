use std::io::{self, BufRead};
use std::ptr;


struct Board {
    fields: Vec<Vec<Field>>,
    done: bool,
}

struct Field {
    num: i32,
    checked: bool,
}

impl Board {
    fn check(&mut self, num:i32) {
        for row in self.fields.iter_mut() {
            for field in row {
                if field.num == num {
                    field.checked = true;
                }
            }
        }
    }

    fn finished(&self) -> bool {
        for i in 0..self.fields.len() {
            let mut column_finished = true;
            let mut row_finished = true;
            for j in 0..self.fields.len() {
                if !self.fields[j][i].checked {
                    column_finished = false;
                }
                if !self.fields[i][j].checked {
                    row_finished = false;
                }
            }
            if row_finished || column_finished {
                return true;
            }
        }
        return false;
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for row in self.fields.iter() {
            for field in row {
                if !field.checked {
                    score = score + field.num;
                }
            }
        }
        return score;
    }

    fn print(&self) {

        for row in self.fields.iter() {
            for field in row {
                if field.checked {
                    print!("X({}) ",field.num);
                    continue;
                }
                print!("O({}) ", field.num);
            }
            print!("\n");
        }
    }
}

fn main() {
    let mut numbers = Vec::new();
    let mut line_number = -1;
    let mut boards:Vec<Board> = Vec::new();
    let mut current_board:*mut Board = ptr::null_mut();
    for line in io::stdin().lock().lines() {
        line_number = line_number + 1;
        let line_str = line.unwrap();
        if line_number == 0 {
            for num in line_str.split(",") {
                numbers.push(num.parse::<i32>().unwrap())
            }
            continue;
        }


        if line_str == "" {
            boards.push(Board{
                fields: Vec::new(),
                done: false,
            });
            let last_board = boards.len()-1;
            current_board = &mut (boards[last_board]);
            continue;
        }

        if current_board.is_null() {
            panic!("no board selected");
        }

        unsafe {
            (*current_board).fields.push(Vec::new());
            for num in line_str.split_whitespace() {
                if num == "" {
                    continue;
                }
                (*current_board).fields[(*current_board).fields.len()-1].push(Field{
                    num:num.parse::<i32>().unwrap(),
                    checked: false,
                });
            }

        }
    }

    let num_boards = boards.len();
    let mut finished_boards = 0;
    for number in numbers.iter() {
        for board in boards.iter_mut() {
            if board.done {
                continue;
            }
            board.check(*number);
            if board.finished() {
                board.done = true;
                finished_boards = finished_boards+1;
                println!("{} out of {} boards finished at score {} with number {}", finished_boards, num_boards, (*number)*board.score(), *number);
                board.print();
                if finished_boards == num_boards {
                    return;
                }
            }
        }
    }
    panic!("No board finished");
}