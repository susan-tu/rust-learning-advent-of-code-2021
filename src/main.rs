use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day_two()
}

fn day_one() -> Result<(), Box<dyn Error>> {
    let file = File::open("/Users/susantu/rust_learning/advent-of-code-2021/inputs/day1.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut depths_vec = Vec::new();
    for line in lines {
        if let Ok(text) = line {
            depths_vec.push(text.parse::<u32>().unwrap());
        }   
    }
    println!("day 1, part 1: {}", calculate_num_depth_increases(&depths_vec[..]));
    println!("day 1, part 2: {}", calculate_windowed_num_depth_increases(&depths_vec[..], 3));
    Ok(())
}

fn day_two() -> Result<(), Box<dyn Error>> {
    let file = File::open("/Users/susantu/rust_learning/advent-of-code-2021/inputs/day2.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut input = Vec::new();
    for line in lines {
        if let Ok(text) = line {
            let parts: Vec<&str> = text.split(" ").collect();
            let direction =  match parts[0] {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => panic!("Unrecognized direction {}", parts[0]) 
            };
            let distance = parts[1].parse::<u32>().unwrap();
            input.push(Move {direction: direction, distance: distance});
        }   
    }
    let pos = calculate_position(&input[..]);
    println!("day 2, part 1: {}", pos.0 * pos.1);
    Ok(())
}


enum Direction {
    Forward,
    Up,
    Down
}

struct Move {
    direction: Direction,
    distance: u32,
}

fn calculate_position(moves: &[Move]) -> (u32, u32) {
    let mut x_pos = 0;
    let mut y_pos = 0;
    for movement in moves {
        match movement.direction {
            Direction::Forward => x_pos += movement.distance,
            Direction::Up => y_pos -= movement.distance,
            Direction::Down => y_pos += movement.distance
        }
    }
    (x_pos, y_pos)
}

fn calculate_windowed_num_depth_increases(depths: &[u32], window_size: usize) -> u32 {
    if depths.len() < window_size {
        panic!("input to calculate_windowed_num_depth_increases must have more than window_size elements");
    }
    let mut prev_sum: u32 = depths[0..window_size].iter().sum();
    let mut increases = 0;
    for n in 1..(depths.len() - window_size + 1) {
        let new_sum = depths[n..(n + window_size)].iter().sum();
        if new_sum > prev_sum {
            increases += 1;
        }
        prev_sum = new_sum;
    }
    increases
}

fn calculate_num_depth_increases(depths: &[u32]) -> u32{
    if depths.len() < 2 {
        panic!("input to calculate_num_depth_increases must have more than 2 elements");
    }
    let mut prev_depth = depths[0];
    let depths_after_first = &depths[1..];
    let mut increases = 0;
    for d in depths_after_first {
        if *d > prev_depth {
            increases += 1;
        }
        prev_depth = *d;
    }
    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_num_depth_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_num_depth_increases(&input[..]), 7)
    }

    #[test]
    fn it_calculates_windowed_num_depth_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_windowed_num_depth_increases(&input[..], 1), 7);
        assert_eq!(calculate_windowed_num_depth_increases(&input[..], 3), 5);
    }

    #[test]
    fn it_calculates_position() {
        let input = vec![
            Move {direction: Direction::Forward, distance: 5},
            Move {direction: Direction::Down, distance: 5},
            Move {direction: Direction::Forward, distance: 8},
            Move {direction: Direction::Up, distance: 3},
            Move {direction: Direction::Down, distance: 8},
            Move {direction: Direction::Forward, distance: 2}
        ];
        assert_eq!(calculate_position(&input[..]), (15, 10))
    }
}
