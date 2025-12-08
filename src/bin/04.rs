use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."; // TODO: Add the test input

#[derive(Debug, Clone)]
struct PaperMap {
    map: Vec<Vec<bool>>,
}

impl PaperMap {
    fn new(input: &Vec<String>) -> PaperMap {
        let mut map = Vec::new();
        for line in input {
            let mut vec1 = Vec::new();
            for ch in line.chars() {
                vec1.push(ch == '@')
            }
            // println!("line {:?} -> {:?}", line, vec1);
            map.push(vec1);
        }
        PaperMap { map }
    }

    fn get(&self, x: i64, y: i64) -> Option<&bool> {
        if x < 0 || y < 0 {
            return None;
        }
        self.map.get(x as usize).and_then(|row| row.get(y as usize))
    }
    fn set(&mut self, x: i64, y: i64, value: bool) -> () {
        self.map.get_mut(x as usize).and_then(|row| {
            row[y as usize] = value;
            Some(value)
        });
    }

    fn adjacent(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for (dx, dy) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let x_pos = (x as i64 + dx);
            let y_pos = (y as i64 + dy);
            let adj_ = self.get(x_pos, y_pos);
            if adj_ == Some(&true) {
                count += 1;
            }
        }
        count
    }

    fn solve_puzzle(&self) -> (usize, PaperMap) {
        let mut new_map = self.clone();
        let mut sum = 0;
        for (x, row) in self.map.iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                if *val {
                    let num = self.adjacent(x, y);
                    if num < 4 {
                        sum += 1;
                        new_map.set(x as i64, y as i64, false);
                    }
                }
            }
        }
        (sum, new_map)
    }

    pub(crate) fn part1(&self) -> usize {
        let (sum, _map) = self.solve_puzzle();
        sum
    }

    pub(crate) fn part2(&self) -> usize {
        let mut sum = 0;
        let mut step = 1;
        let mut map = self;
        let (mut del_sum, mut new_map) = self.solve_puzzle();
        println!("\n part2: {} {} {:?}", step, del_sum, new_map);
        sum += del_sum;
        while step > 0 {
            (del_sum, new_map) = new_map.solve_puzzle();
            step = del_sum;
            sum += del_sum;
            map = &new_map;
            println!("{} {} {:?}", step, sum, new_map);
        }

        sum
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let input = reader.lines().flatten().collect::<Vec<String>>();
        let pm = PaperMap::new(&input);
        // println!("{:?}", pm);

        let answer = pm.part1();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader.lines().flatten().collect::<Vec<String>>();
        let pm = PaperMap::new(&input);
        println!("{:?}", pm);

        let answer = pm.part2();
        Ok(answer)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
