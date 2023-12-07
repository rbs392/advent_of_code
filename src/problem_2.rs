use std::{fs, collections::{HashMap, HashSet}};

use regex::Regex;

#[derive(Debug)]
struct Game {
    game_id: u8,
    red: u8,
    blue: u8,
    green: u8,
}
pub struct CubesCapacity {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}
pub struct Problem2 {
    pub capacity: CubesCapacity,
    pub input_path: String,
}


impl Problem2 {
    fn read_file(&self) -> String {
        fs::read_to_string(&self.input_path).unwrap()
    }

    fn is_impossible_game(&self, game: &Game) -> bool {
        (game.red > self.capacity.red) || 
        (game.blue > self.capacity.blue) || 
        (game.green > self.capacity.green)
    }

    fn get_game(&self, line: &str, regex: &Regex) -> Game {
        let parts = line.split(":").collect::<Vec<&str>>();
        let game_id = parts.first().unwrap().to_string().replace("Game ", "").parse::<u8>().unwrap();
        let draws = parts
            .last()
            .unwrap()
            .split(";")
            .flat_map(|play| {
                regex
                    .find_iter(play)
                    .fold(HashMap::<String, u8>::new(), |mut cur, _match|{
                        let parts = _match.as_str().split(" ").collect::<Vec<&str>>();
                        let (count, color) = (
                            parts.first().unwrap().to_string().parse::<u8>().unwrap(), 
                            parts.last().unwrap().to_string(),
                        );
                        cur.insert(color, count);
                        cur
                    })
            })
            .fold(HashMap::<String, u8>::new(), |mut acc, (key, value)| {
                match acc.get(&key) {
                    Some(v) => {if value > v.to_owned() {acc.insert(key, value);}},
                    None => {acc.insert(key, value);},
                };
                return acc
            });
        Game{
            game_id: game_id,
            red: draws.get("red").unwrap_or(&0).to_owned(),
            blue: draws.get("blue").unwrap_or(&0).to_owned(),
            green: draws.get("green").unwrap_or(&0).to_owned(),
        }
    }

    fn get_games(&self) -> Vec<Game> {
        let binding = self.read_file();
        let regex = Regex::new(r"(\d+)\s(:?red|blue|green)").unwrap();
        binding
            .lines()
            .map(|line| {
                let game = self.get_game(line, &regex);
                game
            })
            .collect::<Vec<Game>>()
    }

    
    pub fn part_1(&self) -> u32 {
        
        let games = self.get_games();
        let impossible_games: HashSet<u32> = HashSet::from_iter(
            games
                .iter()
                .filter(|game| self.is_impossible_game(game))
                .map(|game| (game.game_id as u32))
        );

        let all_games = HashSet::from_iter(games.iter().map(|game| (game.game_id as u32)));
        all_games.difference(&impossible_games).sum::<u32>()
    }

    pub fn part_2(&self) -> u32 {
        let games = self.get_games();
        games.iter().map(|g| {
            g.red as u32 * g.blue as u32 * g.green as u32
        }).sum()
    }
}