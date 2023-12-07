use std::{fs, collections::HashMap};

use regex::Regex;

pub struct Problem5 {
    pub input_path: String
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}
#[derive(Debug, Clone)]
struct Map {
    source: Vec<Range>,
    dest: Vec<Range>,
}

struct Mappings {
    seeds: Vec<u64>,
    seeds_range: Vec<Range>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map, 
    fertilizer_to_water: Map, 
    water_to_light: Map, 
    light_to_temperature: Map, 
    temperature_to_humidity: Map, 
    humidity_to_location: Map, 
}

impl Problem5 {
    fn read_file(&self) -> String {
        fs::read_to_string(&self.input_path).unwrap()
    }

    fn line_to_map(&self, line: &str) -> (Range, Range) {
        let a: Vec<u64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let dest_start = a[0];
        let source_start = a[1];
        let range = a[2];

        (
            Range{start: source_start, end: source_start+range-1},
            Range{start: dest_start, end: dest_start+range-1},
        )

        
    }

    fn build_maps(&self, use_seed_range: bool) -> Mappings {
        let mut seeds = Vec::<u64>::new();
        let mut seeds_range = Vec::<Range>::new();
        let mut seed_to_soil = Map{source: Vec::new(), dest: Vec::new()};
        let mut soil_to_fertilizer  = Map{source: Vec::new(), dest: Vec::new()};
        let mut fertilizer_to_water  = Map{source: Vec::new(), dest: Vec::new()};
        let mut water_to_light = Map{source: Vec::new(), dest: Vec::new()};
        let mut light_to_temperature = Map{source: Vec::new(), dest: Vec::new()};
        let mut temperature_to_humidity = Map{source: Vec::new(), dest: Vec::new()};
        let mut humidity_to_location = Map{source: Vec::new(), dest: Vec::new()};
        
        let info_line_regex = Regex::new(r"[A-z]").unwrap();
        let mut current_info = "";
        
        self.read_file().lines().filter(|line| line.len() > 0).for_each(|line|{
            if info_line_regex.is_match(line){
                current_info = match line {
                    "seed-to-soil map:" => "seed_to_soil",
                    "soil-to-fertilizer map:" => "soil_to_fertilizer",
                    "fertilizer-to-water map:" => "fertilizer_to_water",
                    "water-to-light map:" => "water_to_light",
                    "light-to-temperature map:" => "light_to_temperature",
                    "temperature-to-humidity map:" => "temperature_to_humidity",
                    "humidity-to-location map:" => "humidity_to_location",
                    _ => {
                        if line.starts_with("seeds:") {
                            let tmp_seeds: Vec<u64> = line.replace("seeds: ", "").split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
                            if use_seed_range{
                                let (clone, lag) = tmp_seeds.iter().fold((Vec::<u64>::new(), Vec::<u64>::new()), |mut acc, cur| {
                                    let (mut clone, mut lag) = acc;
                                    if clone.len() > 0 {
                                        lag.push(*cur)
                                    }
                                    if clone.len() < tmp_seeds.len()-1{
                                        clone.push(*cur);
                                    } 
                                    (clone, lag)
                                });
                                seeds_range = clone.iter().zip(lag.iter()).map(|(start, end)|{
                                    Range{start: *start, end: *end}
                                }).collect();
                                

                            } else {
                                seeds = tmp_seeds;
                            }
                        }
                        ""
                    },
                };
            } else {
                let (source, dest) = self.line_to_map(line);
                match current_info {
                    "seed_to_soil" => {
                        seed_to_soil.source.push(source);
                        seed_to_soil.dest.push(dest);
                    },
                    "soil_to_fertilizer" => {
                        soil_to_fertilizer.source.push(source);
                        soil_to_fertilizer.dest.push(dest);
                    },
                    "fertilizer_to_water" => {
                        fertilizer_to_water.source.push(source);
                        fertilizer_to_water.dest.push(dest);
                    },
                    "water_to_light" => {
                        water_to_light.source.push(source);
                        water_to_light.dest.push(dest);
                    },
                    "light_to_temperature" => {
                        light_to_temperature.source.push(source);
                        light_to_temperature.dest.push(dest);
                    },
                    "temperature_to_humidity" => {
                        temperature_to_humidity.source.push(source);
                        temperature_to_humidity.dest.push(dest);
                    },
                    "humidity_to_location" => {
                        humidity_to_location.source.push(source);
                        humidity_to_location.dest.push(dest);
                    },
                    _ => ()
                }
            }
            
        });
        Mappings{
            seeds,
            seeds_range,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }

    }
    pub fn part_1(&self) -> u64{
        let mappings = self.build_maps(false);
        mappings.seeds.iter().map(|seed| {
            let location = [
                &mappings.seed_to_soil,
                &mappings.soil_to_fertilizer,
                &mappings.fertilizer_to_water,
                &mappings.water_to_light,
                &mappings.light_to_temperature,
                &mappings.temperature_to_humidity,
                &mappings.humidity_to_location,
            ].iter().fold(seed.to_owned(), |acc, cur| {
                let index = cur.source.iter().position(|x| (x.start <= acc) && (acc <= x.end));
                match index {
                    Some(x) => {
                        let s_range = &cur.source[x];
                        let inc = acc - s_range.start;
                        &cur.dest[x].start + inc
                    },
                    None => acc
                }
            });
            location
        }).min().unwrap().to_owned()
    }

    pub fn part_2(&self) -> u64{
        let mappings = self.build_maps(true);
        let mut location_end_list = mappings.humidity_to_location.dest
            .iter()
            .fold(Vec::<(u64, u8)>::new(), |mut acc, cur|{
                match acc.last() {
                    None => {
                        acc.push((cur.start, 0))
                    }
                    Some((_, i)) => {
                        acc.push((cur.start, i+1))
                    },
                }
                acc
            });
        location_end_list.sort_by_key(|(x, _)| *x);

        println!("{:?}", mappings.humidity_to_location.dest.len());

        let transforms = [
            &mappings.temperature_to_humidity,
            &mappings.light_to_temperature,
            &mappings.water_to_light,
            &mappings.fertilizer_to_water,
            &mappings.soil_to_fertilizer,
            &mappings.seed_to_soil,
        ];

        println!("humidity_to_location={:?}", mappings.humidity_to_location);
        println!("location_end_list={:?}", location_end_list);

        location_end_list.iter().map(|(_, x)| x).for_each(|i|{
            let humidity_range = mappings.humidity_to_location.source[*i as usize];
            let mut tmp = humidity_range.start;
            println!("humidity={:?}", tmp);
            let lowest_seed = transforms.iter().map(|cur|{
                for d in *cur.dest{

                }
            });

            println!("lowest_seed={:?}", lowest_seed);
        });

        1
    }
}