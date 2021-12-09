use std::io::{self, BufRead};
use std::cmp;
use rayon::prelude::*;



#[derive(Debug, Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Valpoint {
    p: Point,
    val: i32,
}


fn main() {
    let mut lines:Vec<Line> = Vec::new();
    for line in io::stdin().lock().lines() {
        let line_str = line.unwrap();
        let mut i = 0;
        let mut p1:Point = Point{x:0,y:0};
        let mut p2:Point = Point{x:0,y:0};
        for point_str in line_str.split(" -> ") {
            if i == 0 {
                p1 = parse_coord(point_str.to_string());
            } else {
                p2 = parse_coord(point_str.to_string());
            }
            i = i + 1;
        }
        lines.push(Line{
            p1: p1,
            p2: p2,
        })
    }

    println!("{} lines parsed", lines.len());

    let mut covered_points:Vec<Valpoint> = Vec::new();
    for line in lines.iter() {

        let dx = line.p2.x-line.p1.x;
        let dy = line.p2.y-line.p1.y;

        for i in 0..=cmp::max(dx.abs(), dy.abs()) {
            let mut x = line.p1.x + i;
            if dx < 0 {
                x = line.p1.x - i;
            }
            if dx == 0 {
                x = line.p1.x;
            }

            let mut y = line.p1.y + i;
            if dy < 0 {
                y = line.p1.y - i;
            }
            if dy == 0 {
                y = line.p1.y;
            }
            cover_point(&mut covered_points, x, y);
        }
    }
    println!("{} points covered", covered_points.len());

    let mut dangerous_points = 0;
    for p in covered_points.iter() {
        if p.val > 1 {
            dangerous_points = dangerous_points + 1;
        }
    }

    println!("{} dangerous points found", dangerous_points);
}

fn cover_point(covered_points:&mut Vec<Valpoint>, x:i32, y:i32) {
    if !covered_points.par_iter().any(|p| p.p.x == x && p.p.y == y) {
        covered_points.push(Valpoint{
            p: Point{
                x: x,
                y: y,
            },
            val: 1,
        });
        return;
    };
    covered_points.par_iter_mut().for_each(|p| {
        if p.p.x == x && p.p.y == y {
            p.val = p.val + 1;
        }
    });
}

fn parse_coord(point_str:String) -> Point {
    let mut p:Point = Point{x:0,y:0};
    let mut j = 0;

    for coord_str in point_str.split(",") {
        if j == 0 {
            p.x=coord_str.parse::<i32>().unwrap()
        } else {
            p.y=coord_str.parse::<i32>().unwrap()
        }
        j = j + 1;
    }
    return p;
}