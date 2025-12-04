use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

#[derive(Debug)]
struct Sequence {
    start: String,
    end: String,
}

impl Sequence {
    fn new(input: &String) -> Result<Sequence> {
        let start_stop = input.split('-').collect::<Vec<_>>();
        // println!("input: {:?} -- {:?}", input, start_stop);
        let start = start_stop[0].to_string();
        let end = start_stop[1].to_string();
        Ok(Self {
            start,
            end
        })
    }

    fn valid_id(s: &String) -> bool {
        let (b, e) = s.split_at(s.len()/2);
        !b.eq(e)
    }
    fn invalid_id(s: &String) -> bool {
        !Self::valid_id(s)
    }
    fn second_invalid_id(s: &String) -> bool {
        println!("second_invalid_id({:?})", s);
        let max_part = s.len()/2;
        for i in 1..=max_part {
            // println!("{:?}", i);
            if s.len() % i == 0 {
                let parts = &s.chars().chunks(i);
                let s_parts = parts.into_iter().map(|c| c.collect::<String>()).collect::<Vec<_>>();
                // println!("{:?}", s_parts);
                let first = s_parts[0].clone();
                let invalid = s_parts.iter().all(|item| *item == first);
                // println!("{:?}", invalid);
                if invalid {
                    return true;
                }
            }
        }
        false
    }

    fn range(&self) -> Vec<String> {
        let mut rng = vec![];
        let start = self.start.parse::<usize>().unwrap();
        let end = self.end.parse::<usize>().unwrap();
        // println!("start: {:?} end - {:?}", start, end);
        for n in start..=end {
            let string = n.to_string();
            rng.push(string);
        }
        rng
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut sum = 0;
        let lines = reader.lines().flatten().collect::<Vec<_>>();
        let input_string = &lines[0];
        let input = input_string.split(',');
        for sequence in input {
            let seq = Sequence::new(&sequence.to_string())?;
            let range = seq.range();
            let invalids = range.into_iter().filter(Sequence::invalid_id);
            for invalid in invalids {
                sum += invalid.parse::<usize>()?;
            }
        }
        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum = 0;
        let lines = reader.lines().flatten().collect::<Vec<_>>();
        let input_string = &lines[0];
        let input = input_string.split(',');
        for sequence in input {
            println!("{}", sequence);
            let seq = Sequence::new(&sequence.to_string())?;
            let range = seq.range();
            let invalids = range.into_iter().filter(Sequence::second_invalid_id);
            for invalid in invalids {
                println!("{}", invalid);
                sum += invalid.parse::<usize>()?;
            }
        }
        Ok(sum)

    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
