extern crate regex;

use std::{collections::HashMap, fs::File, io::prelude::*, ops::Sub};

const INPUT: &'static str = "../inputs/day04.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut lines: Vec<String> = input.lines().map(|v| v.to_owned()).collect();
    lines.sort_unstable();

    let line_re =
        regex::Regex::new(r"[\[]([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)[\]] (.*)").unwrap();
    let guard_re = regex::Regex::new(r"Guard #([0-9]+) begins shift").unwrap();
    let mut guard_id = None;
    let mut guard_asleep_at = None;
    let mut guard_asleep_minutes = HashMap::new();
    for line in &lines {
        // println!("line: \"{}\"", line);
        let line_cap = line_re.captures(line).unwrap();
        // println!("cap: {:?}", cap);
        let year: u16 = line_cap[1].parse().unwrap();
        let month: u8 = line_cap[2].parse().unwrap();
        let day: u8 = line_cap[3].parse().unwrap();
        let hour: u8 = line_cap[4].parse().unwrap();
        let minute: u8 = line_cap[5].parse().unwrap();
        let text = &line_cap[6];
        if let Some(guard_cap) = guard_re.captures(text) {
            let guard_id_: u32 = guard_cap[1].parse().unwrap();
            guard_id = Some(guard_id_);
        } else if text == "falls asleep" {
            guard_asleep_at = Some(DateTime::new(year, month, day, hour, minute));
        } else if text == "wakes up" {
            let minutes_asleep =
                (DateTime::new(year, month, day, hour, minute) - guard_asleep_at.take().unwrap());
            if !guard_asleep_minutes.contains_key(&guard_id.unwrap()) {
                guard_asleep_minutes.insert(guard_id.unwrap(), 0);
            }
            *guard_asleep_minutes.get_mut(&guard_id.unwrap()).unwrap() += minutes_asleep;
        } else {
            panic!("unexpected text: {:?}", text);
        }
    }
    // println!("guard_asleep_minutes: {:#?}", guard_asleep_minutes);
    let (the_guard_id, the_minutes) = guard_asleep_minutes.iter().fold(
        (0, 0),
        |(max_guard_id, max_minutes), (guard_id, minutes)| {
            if *minutes > max_minutes {
                (*guard_id, *minutes)
            } else {
                (max_guard_id, max_minutes)
            }
        },
    );

    println!(
        "the_guard_id: {:?}, the_minutes: {:?}",
        the_guard_id, the_minutes
    );

    let mut minutes_asleep = [0_usize; 24 * 60];
    let mut guard_id = None;
    let mut guard_asleep_at = None;
    for line in &lines {
        // println!("line: \"{}\"", line);
        let line_cap = line_re.captures(line).unwrap();
        // println!("cap: {:?}", cap);
        let year: u16 = line_cap[1].parse().unwrap();
        let month: u8 = line_cap[2].parse().unwrap();
        let day: u8 = line_cap[3].parse().unwrap();
        let hour: u8 = line_cap[4].parse().unwrap();
        let minute: u8 = line_cap[5].parse().unwrap();
        let text = &line_cap[6];
        if let Some(guard_cap) = guard_re.captures(text) {
            let guard_id_: u32 = guard_cap[1].parse().unwrap();
            guard_id = Some(guard_id_);
            guard_asleep_at = None;
        } else if text == "falls asleep" {
            assert!(guard_asleep_at.is_none());
            guard_asleep_at = Some(DateTime::new(year, month, day, hour, minute));
        } else if text == "wakes up" {
            assert!(guard_asleep_at.is_some());
            let guard_asleep_at = guard_asleep_at.take().unwrap();
            if guard_id.unwrap() == the_guard_id {
                assert!(
                    guard_asleep_at.is_same_day_as(DateTime::new(year, month, day, hour, minute))
                );
                for i in (guard_asleep_at.hour as usize * 60 + guard_asleep_at.minute as usize)
                    ..(hour as usize * 60 + minute as usize)
                {
                    minutes_asleep[i] += 1;
                }
            }
        } else {
            panic!("unexpected text: {:?}", text);
        }
    }

    let (max_i, max) = minutes_asleep
        .iter()
        .enumerate()
        .fold(
            (0, 0),
            |(max_i, max), (i, v)| {
                if *v > max {
                    (i, *v)
                } else {
                    (max_i, max)
                }
            },
        );

    println!("max_i: {:?}, max: {:?}", max_i, max);
    println!("answer: {}", the_guard_id * max_i as u32)
}

/// Minutes since 0000-01-01 00:00
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl DateTime {
    fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8) -> Self {
        DateTime {
            year,
            month,
            day,
            hour,
            minute,
        }
    }

    fn is_same_day_as(self, rhs: DateTime) -> bool {
        self.year == rhs.year && self.month == rhs.month && self.day == rhs.day
    }
}

impl Sub<DateTime> for DateTime {
    type Output = u64;
    fn sub(self, rhs: DateTime) -> Self::Output {
        if self.is_same_day_as(rhs) {
            (self.hour as u64 * 60 + self.minute as u64)
                - (rhs.hour as u64 * 60 + rhs.minute as u64)
        } else {
            panic!("different day");
        }
    }
}
