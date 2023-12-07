use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Regex, Matches};

lazy_static! {
    static ref DIGITS_REGEX: Regex = Regex::new("\\d").unwrap();

    static ref NUMBER_ENCODING: HashMap<String, EncodeValue> = HashMap::from([
        ("zero".to_string(), EncodeValue{value: 0, regex: Regex::new("zero").unwrap()}),
        ("one".to_string(), EncodeValue{value: 1, regex: Regex::new("one").unwrap()}),
        ("two".to_string(), EncodeValue{value: 2, regex: Regex::new("two").unwrap()}),
        ("three".to_string(), EncodeValue{value: 3, regex: Regex::new("three").unwrap()}),
        ("four".to_string(), EncodeValue{value: 4, regex: Regex::new("four").unwrap()}),
        ("five".to_string(), EncodeValue{value: 5, regex: Regex::new("five").unwrap()}),
        ("six".to_string(), EncodeValue{value: 6, regex: Regex::new("six").unwrap()}),
        ("seven".to_string(), EncodeValue{value: 7, regex: Regex::new("seven").unwrap()}),
        ("eight".to_string(), EncodeValue{value: 8, regex: Regex::new("eight").unwrap()}),
        ("nine".to_string(), EncodeValue{value: 9, regex: Regex::new("nine").unwrap()}),
    ]);
}

struct EncodeValue {
    value: u32,
    regex: Regex,
}



pub struct Problem1 {
    pub input_path: String,
}


impl Problem1 {
    fn read_file(&self) -> String {
        std::fs::read_to_string(&self.input_path).unwrap()
    }

    fn convert_matches_to_vec(&self, matches: Matches<'_, '_>, value: Option<u32>) -> Vec<(usize, u32)> {
        matches
            .map(|_match| {
                let v = match value {
                    None => _match.as_str().parse::<u32>().unwrap(),
                    Some(x) => x,
                };
                (_match.start(), v)
            })
            .collect::<Vec<(usize, u32)>>()
    }

    fn get_digit_matches(&self, line: &str) -> Vec<(usize, u32)>{
        self.convert_matches_to_vec(DIGITS_REGEX.find_iter(line), None)
    }

    fn to_number(&self, mut matches: Vec<(usize, u32)>) -> u32 {
        matches.sort_by_key(|x| x.0);
        let sorted_res = matches
            .iter()
            .map(|_match| _match.1)
            .collect::<Vec<u32>>();
        let (first, last) = (sorted_res.first().unwrap(), sorted_res.last().unwrap());
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }

    fn get_number_encoded_matches(&self, line: &str) -> Vec<(usize, u32)> {
        NUMBER_ENCODING
            .iter()
            .filter(|(key, _)| line.contains(key.as_str()))
            .flat_map(|(_, value)| {
                self.convert_matches_to_vec(value.regex.find_iter(line), Some(value.value))
            })
            .collect::<Vec<(usize, u32)>>()
    }

    pub fn part_1(&self) -> u32 {
        self.read_file()
            .lines()
            .map(|line| self.get_digit_matches(line))
            .map(|sorted_res| self.to_number(sorted_res))
            .sum()
    }

    pub fn part_2(&self) -> u32 {
        self.read_file()
            .lines()
            .map(|line| {
                let mut matches = self.get_digit_matches(line);
                matches.append(&mut self.get_number_encoded_matches(line));
                self.to_number(matches)
            })
            .sum()
    }
}
