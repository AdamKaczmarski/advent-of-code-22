use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, fs};

#[derive(Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
enum Category {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Clone)]
struct Seed {
    id: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temp: usize,
    humidity: usize,
    location: usize,
}
impl Seed {
    fn set_position_to_category(&mut self, category: &Category, position: usize) {
        match category {
            Category::SeedToSoil => self.soil = position,
            Category::SoilToFertilizer => self.fertilizer = position,
            Category::FertilizerToWater => self.water = position,
            Category::WaterToLight => self.light = position,
            Category::LightToTemperature => self.temp = position,
            Category::TemperatureToHumidity => self.humidity = position,
            Category::HumidityToLocation => self.location = position,
        }
    }

    fn get_relevant_position_for_category(&self, category: &Category) -> usize {
        let val = match category {
            Category::SeedToSoil => self.id,
            Category::SoilToFertilizer => self.soil,
            Category::FertilizerToWater => self.fertilizer,
            Category::WaterToLight => self.water,
            Category::LightToTemperature => self.light,
            Category::TemperatureToHumidity => self.temp,
            Category::HumidityToLocation => self.humidity,
        };
        return val;
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
struct MapRange {
    destination_range: RangeTuple,
    source_range: RangeTuple,
}
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
struct RangeTuple {
    start: usize,
    finish: usize,
}
impl RangeTuple {
    fn new(start: usize, finish: usize) -> Self {
        return RangeTuple { start, finish };
    }
    fn is_in_range(&self, num: &usize) -> bool {
        return self.start <= *num && self.finish >= *num;
    }
}

impl MapRange {
    fn new(input: &str) -> Self {
        let input = input.trim().split(" ").collect_vec();
        let range_len = input.get(2).unwrap().parse::<usize>().unwrap();
        let dest_start = input.get(0).unwrap().parse::<usize>().unwrap();
        let src_start = input.get(1).unwrap().parse::<usize>().unwrap();

        return Self {
            destination_range: RangeTuple::new(dest_start, dest_start + range_len - 1),
            source_range: RangeTuple::new(src_start, src_start + range_len - 1),
        };
    }
    fn get_dest_pos_from_src(&self, src_pos: usize) -> InRangeDecisionPosition {
        if self.source_range.is_in_range(&src_pos) {
            let offset = self.source_range.finish - src_pos;
            return InRangeDecisionPosition {
                is_in_range: true,
                position: self.destination_range.finish - offset,
            };
        }
        return InRangeDecisionPosition {
            is_in_range: false,
            position: src_pos,
        };
    }
}
#[derive(Clone, Debug)]
struct InRangeDecisionPosition {
    is_in_range: bool,
    position: usize,
}

/*
 * 1. Read the first line to get the seeds
 * 2. A Map that will map each categroy (seed-to-soil to it's starting and ending location)
 * 3. Based on the map Create a Seed struct.
 * ?4. Maintain while parsing it all for each seed just maintain min location
 * ?4. Find the lowerst location of seed.
 *
 * OR
 *
 * 2. A Map that will map each categroy (seed-to-soil to it's starting and ending location)
 * 2a. While mapping just count the seed's number according to category
 *
 */
fn parse_seed_ids(line: &str, seeds_vec: &mut Vec<Seed>) {
    let colon_idx = line.find(":").unwrap();
    let line = &line[colon_idx + 1..];
    let trimmed_line = line.trim();
    for id in trimmed_line.split(" ") {
        let id = id.parse::<usize>().unwrap();
        seeds_vec.push(Seed {
            id,
            soil: 0,
            fertilizer: 0,
            water: 0,
            light: 0,
            temp: 0,
            humidity: 0,
            location: 0,
        })
    }
}

fn match_category(line: &str) -> Category {
    return match line {
        "seed-to-soil map:" => Category::SeedToSoil,
        "soil-to-fertilizer map:" => Category::SoilToFertilizer,
        "fertilizer-to-water map:" => Category::FertilizerToWater,
        "water-to-light map:" => Category::WaterToLight,
        "light-to-temperature map:" => Category::LightToTemperature,
        "temperature-to-humidity map:" => Category::TemperatureToHumidity,
        "humidity-to-location map:" => Category::HumidityToLocation,
        _ => panic!("wrong line here {}", line),
    };
}

fn main() -> Result<()> {
    let input_lines = fs::read_to_string("./inputs/day5.prod")?;
    let input_lines = input_lines.trim().split("\n\n").collect_vec();
    let mut seeds: Vec<Seed> = Vec::new();
    parse_seed_ids(input_lines.get(0).unwrap(), &mut seeds);
    let mut category_maps: HashMap<Category, Vec<MapRange>> = HashMap::new();
    input_lines.iter().skip(1).for_each(|map_and_data| {
        let map = map_and_data.split("\n").collect_vec();
        let category = match_category(map.get(0).unwrap());
        map.iter().skip(1).for_each(|range| {
            let map_range: MapRange = MapRange::new(range);
            match category_maps.get_mut(&category) {
                Some(ranges) => ranges.push(map_range),
                None => {
                    let _ = category_maps.insert(category.clone(), vec![map_range]);
                }
            }
        })
    });
    let mut min_location: usize = usize::MAX;
    seeds.iter_mut().for_each(|seed| {
        category_maps
            .iter_mut()
            .sorted()
            .for_each(|(category, ranges)| {
                for range in ranges {
                    let in_range_pos: InRangeDecisionPosition = range
                        .get_dest_pos_from_src(seed.get_relevant_position_for_category(category));
                    if in_range_pos.is_in_range {
                        //If it is in the range break the loop
                        seed.set_position_to_category(category, in_range_pos.position);
                        break;
                    }
                    //Loop over all ranges to check if it is there
                    // It is going to do some overwrite but that's to be though about later
                    seed.set_position_to_category(category, in_range_pos.position);
                }
                if category == &Category::HumidityToLocation {
                    min_location = std::cmp::min(seed.location, min_location);
                }
            });
    });
    // for seed in seeds{
    // println!("{:?}", seed);
    // }
    //
    println!("Min location: {}", min_location);

    Ok(())
}

impl Default for Seed {
    fn default() -> Self {
        return Seed {
            id: 0,
            soil: 0,
            fertilizer: 0,
            water: 0,
            light: 0,
            temp: 0,
            humidity: 0,
            location: 0,
        };
    }
}
