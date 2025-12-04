use std::collections::{HashMap, HashSet};
use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::iproduct;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
"; // TODO: Add the test input

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn new(batteries: String) -> Bank {
        let batteries: Vec<u8> = batteries
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        Bank { batteries }
    }

    fn max_voltage(&self) -> usize {
        let first = self.batteries.len() - 2;
        let last = self.batteries.len() - 1;
        let mut max: usize = 0;
        for i1 in 0..=first {
            for i2 in 1..=last {
                if i1 >= i2 {
                    continue;
                }
                let fv = self.batteries[i1] as usize;
                let lv = self.batteries[i2] as usize;
                let swaped = fv * 10 + lv;
                if swaped > max {
                    max = swaped;
                }
            }
        }
        max
    }
    fn max_voltage12(&self) -> usize {
        // It’s really easy. Just look for the first instance of the largest digit
        // in digits[0:-11], then if that digit was at index x look in digits[x+1:-10]
        // for the next one and repeat. In 12 simple iterations you can just directly
        // read out all 12 digits. Each iteration is O(n), there is a constant number
        // of them, so the final is also O(n). It’s very fast, it only takes a few lines
        // to implement. You can use the same algorithm for part one of course,
        // but there you just need two iterations.

        let len = self.batteries.len();
        let swaps: usize = 12;
        let mut max: usize = 0;
        let mut index = 0;
        println!("max_voltage12() {:?} {}", self.batteries, len);
        for m in 0..swaps {
            let after_pos = (len - swaps + m);
            let mut lm = 0u8;
            let mut li = 0usize;
            //println!("swap: {} [{}..{}]", m, index, after_pos);
            for i in index..=after_pos {
                let local_voltage = self.batteries[i];
                if lm < local_voltage {
                    lm = local_voltage;
                    li = i;
                }
            }
            max += (lm as usize) * 10usize.pow((swaps - m - 1) as u32);
            // println!("max number {}: {}", max, lm);
            index = li + 1;

        }
        println!("max voltage = {:?} -> {:?}", self.batteries, max);
        max
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let input_strings = reader.lines().flatten().collect::<Vec<String>>();
        let input_batteries = input_strings
            .iter()
            .map(|s| Bank::new(s.to_string()))
            .collect::<Vec<Bank>>();
        let sum = input_batteries
            .iter()
            .map(|b| b.max_voltage())
            .sum::<usize>();

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input_strings = reader.lines().flatten().collect::<Vec<String>>();
        println!("input: {:?}", input_strings);
        let input_batteries = input_strings
            .iter()
            .map(|s| Bank::new(s.to_string()))
            .collect::<Vec<Bank>>();
        println!("input: {:?}", input_batteries);
        let sum = input_batteries
            .iter()
            .map(|b| b.max_voltage12())
            .sum::<usize>();

        Ok(sum)

    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);


    Ok(())
}
