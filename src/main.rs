mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;

use problem_1::Problem1;
use problem_2::{Problem2, CubesCapacity};
use problem_3::Problem3;
use problem_4::Problem4;




fn main() {
    let prob_1 = Problem1{input_path: "resources/puzzle_1".to_string()};
    println!("problem1: part1 = {}", prob_1.part_1());
    println!("problem1: part2 = {}", prob_1.part_2());


    let prob_2 = Problem2{
        capacity: CubesCapacity{red: 12, green: 13, blue: 14},
        input_path: "resources/puzzle_2".to_string(),
    };
    println!("problem2: part1 = {}", prob_2.part_1());
    println!("problem2: part2 = {}", prob_2.part_2());


    let prob_3 = Problem3{input_path: "resources/puzzle_3".to_string()};
    println!("problem3: part1 = {}", prob_3.part_1());
    println!("problem3: part2 = {}", prob_3.part_2());


    let prob_4 = Problem4{input_path: "resources/puzzle_4".to_string()};
    println!("problem4: part1 = {}", prob_4.part_1());
    println!("problem4: part2 = {}", prob_4.part_2());
}
