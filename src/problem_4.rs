use std::{fs, collections::{HashMap, VecDeque}};

use regex::Regex;

pub struct Problem4 {
    pub input_path: String
}

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    id: u32,
    matches: u32
}


impl Problem4 {
    fn read_file(&self) -> String {
        fs::read_to_string(&self.input_path).unwrap()
    }

    fn convert_to_num(&self, text: &str) -> Vec<u8> {
        text.split(" ")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<u8>().unwrap())
            .collect()
    }

    fn get_cards(&self) -> Vec<Card> {
        let card_regex = Regex::new(r"\d+").unwrap();
        self.read_file().lines().map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let card_id = card_regex.find(parts.first().unwrap()).unwrap().as_str().parse().unwrap();
            let number_parts: Vec<&str> = parts.last().unwrap().split("|").collect();
            let winning_numbers: Vec<u8> = self.convert_to_num(number_parts.first().unwrap());
            let elf_numbers: Vec<u8> = self.convert_to_num(number_parts.last().unwrap());
            let matches = elf_numbers.iter()
                .filter(|x| winning_numbers.contains(x))
                .count();
            Card{id: card_id, matches: matches as u32}
        }).collect()
    }

    pub fn part_1(&self) -> u32 {
        self.get_cards()
            .iter()
            .map(|card| card.matches)
            .filter(|x| *x>0)
            .map(|x| (2 as u32).pow((x-1) as u32))
            .sum()
    }

    pub fn part_2(&self) -> u32 {
        let card_map = self.get_cards().iter().fold(HashMap::<u32, Card>::new(), |mut acc, cur| {
            acc.insert(cur.id, cur.to_owned());
            acc
        });

        let mut stack = VecDeque::from(self.get_cards());
        let mut total = 0;
        while stack.len() > 0 {
            total += 1;
            let card = stack.pop_front().unwrap();
            for i in 1..(card.matches+1){
                let next_card_id = card.id + i;
                match card_map.get(&next_card_id) {
                    Some(x) => stack.push_back(x.clone()),
                    None => ()
                }
            }
        }
        
        return total;
    }
}