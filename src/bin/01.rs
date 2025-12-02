use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;
use regex::Regex;
use lazy_static::lazy_static;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"; // TODO: Add the test input
const TEST2: &str = "\
L50
R1000
"; // TODO: Add the test input

#[derive(Debug)]
enum Step {
    Left(u32),
    Right(u32),
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut pos: i64 = 50;
        let re = Regex::new(r"([LR])(\d+)")?;
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let all_lines = reader.lines().flatten();
        for line in all_lines {
            let cap = re.captures(&line);
            let step = if let Some(cap_) = &cap {
                let steps = match cap_.get(2) {
                    Some(m) => {m.as_str().parse::<u32>()?},
                    _ => 0
                };
                if let Some(s) = cap_.get(1) {
                    if s.as_str() == "R" {
                        Step::Right(steps)
                    } else {
                        Step::Left(steps)
                    }
                } else { continue }
            } else { continue };

            // println!("Pos: {} -> {:?}", pos, step);
            match step {
                Step::Left(s) => { pos = pos - (s as i64)}
                Step::Right(s) => { pos = pos + (s as i64)}
            }
            // println!("Pos_2: {} -> {:?}", pos, step);
            pos = pos.rem_euclid(100);
            println!("Pos: {} -> {:?}", pos, step);
            if pos == 0 {
                answer += 1;
            }
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: i64 = 50;
        let mut zero_passed: usize = 0;
        let re = Regex::new(r"([LR])(\d+)")?;

        let all_lines = reader.lines().flatten();
        for line in all_lines {
            let cap = re.captures(&line);
            let step = if let Some(cap_) = &cap {
                let steps = match cap_.get(2) {
                    Some(m) => { m.as_str().parse::<u32>()? },
                    _ => 0
                };
                if let Some(s) = cap_.get(1) {
                    if s.as_str() == "R" {
                        Step::Right(steps)
                    } else {
                        Step::Left(steps)
                    }
                } else { continue }
            } else { continue };
            match step {
                Step::Left(mut value) => {
                    if value > 100 {
                        zero_passed += value as usize / 100;
                        value %= 100;
                    }
                    let is_zero = dial == 0;
                    dial -= value as i64;
                    if !is_zero && dial <= 0 {
                        zero_passed += 1;
                    }
                    if dial < 0 {
                        dial += 100;
                    }
                }
                Step::Right(mut value) => {
                    if value > 100 {
                        zero_passed += value as usize / 100;
                        value %= 100;
                    }
                    dial += value as i64;
                    if dial > 99 {
                        zero_passed += 1;
                        dial %= 100;
                    }

                }
            }
        }
        Ok(zero_passed)
    }
    //
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    println!("\n=== Part 2: many ===");
    assert_eq!(11, part2(BufReader::new(TEST2.as_bytes()))?);
    //
    println!("\n=== Part 2: real ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
